use chrono::{DateTime, Utc};
use chrono_tz::Europe::Helsinki;
use quick_xml::{events::Event, Reader};
use reqwest_middleware::{ClientWithMiddleware, Result as ReqwestResult};
use rocket::State;
use serde::Serialize;

use crate::utils::map_degrees_to_compass;

pub async fn fetch_xml(
    (weather_station_location, url): (&str, &str),
    reqwest_client: &State<ClientWithMiddleware>,
) -> ReqwestResult<String> {
    println!("Fetching weather data for {}", weather_station_location);
    let utc: DateTime<Utc> = Utc::now();
    let utc_minus_1_hour = utc - chrono::Duration::hours(1);
    // Format as 2021-05-01T12:00:00Z
    let formatted_time = utc_minus_1_hour.format("%Y-%m-%dT%H:%M:%SZ");
    let url = format!("{}&starttime={}", url, formatted_time);
    let response = reqwest_client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

enum ParameterType {
    WindSpeed,
    Temperature,
    WindDirection,
    WindGustSpeed,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherData {
    pub time_of_observation: String,
    pub wind_speed_meters_per_second: String,
    pub temperature_celsius: String,
    pub wind_direction: String,
    pub wind_gust_speed_meters_per_second: String,
}

pub fn parse_latest_weather_data(xml: &str) -> Option<WeatherData> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut time_of_observation: Option<String> = None;
    let mut current_wind_speed: Option<String> = None;
    let mut current_temperature: Option<String> = None;
    let mut current_wind_direction: Option<String> = None;
    let mut current_wind_gust_speed: Option<String> = None;
    let mut latest_data: Option<WeatherData> = None;
    let mut current_parameter: Option<ParameterType> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"BsWfs:ParameterName" => match reader.read_text(e.name()) {
                    Ok(contents) => {
                        current_parameter = match contents.to_string().as_str() {
                            "WindSpeedMS" => Some(ParameterType::WindSpeed),
                            "Temperature" => Some(ParameterType::Temperature),
                            "WindDirection" => Some(ParameterType::WindDirection),
                            "WindGust" => Some(ParameterType::WindGustSpeed),
                            _ => None,
                        };
                    }
                    Err(e) => {
                        println!("Failed to read text: {}", e);
                        continue;
                    }
                },
                b"BsWfs:ParameterValue" => match reader.read_text(e.name()) {
                    Ok(contents) => {
                        let contents = contents.to_string();
                        match current_parameter {
                            Some(ParameterType::WindSpeed) => {
                                current_wind_speed = Some(contents);
                            }
                            Some(ParameterType::Temperature) => {
                                current_temperature = Some(contents);
                            }
                            Some(ParameterType::WindDirection) => {
                                let degrees: f32 = match contents.parse() {
                                    Ok(degrees) => degrees,
                                    Err(_) => {
                                        println!("Failed to parse wind direction: {}", contents);
                                        continue;
                                    }
                                };
                                let compass_direction =
                                    match map_degrees_to_compass::convert(degrees) {
                                        Some(direction) => direction.to_string(),
                                        None => "Unknown".to_string(),
                                    };
                                current_wind_direction = Some(compass_direction);
                            }
                            Some(ParameterType::WindGustSpeed) => {
                                current_wind_gust_speed = Some(contents);
                            }
                            _ => (),
                        }
                    }
                    Err(e) => {
                        println!("Failed to read text: {}", e);
                        continue;
                    }
                },
                b"BsWfs:Time" => {
                    let contents = reader.read_text(e.name());
                    match contents {
                        Ok(text) => match text.parse::<DateTime<Utc>>() {
                            Ok(dt) => {
                                let dt_helsinki = dt
                                    .with_timezone(&Helsinki)
                                    .format("%d/%m/%Y %T")
                                    .to_string();
                                time_of_observation = Some(dt_helsinki);
                            }
                            Err(err) => {
                                println!("Failed to parse time: {}", err);
                                continue;
                            }
                        },
                        Err(e) => {
                            // Handle the error from reading the text
                            println!("Failed to read text: {}", e);
                            continue;
                        }
                    }
                }
                _ => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    if let (
        Some(time_of_observation),
        Some(wind_speed),
        Some(temperature),
        Some(wind_direction),
        Some(wind_gust_speed),
    ) = (
        time_of_observation,
        current_wind_speed,
        current_temperature,
        current_wind_direction,
        current_wind_gust_speed,
    ) {
        latest_data = Some(WeatherData {
            time_of_observation,
            wind_speed_meters_per_second: wind_speed,
            temperature_celsius: temperature,
            wind_direction,
            wind_gust_speed_meters_per_second: wind_gust_speed,
        });
    }

    latest_data
}
