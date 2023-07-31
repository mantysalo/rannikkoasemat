mod stations;

use std::collections::HashMap;

use crate::stations::stations::{create_station_hashmap, WeatherLocation};
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Helsinki;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_middleware::{ClientBuilder, Result as ReqwestResult};
use rocket::fs::FileServer;
use rocket::response::status;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::Serialize;
#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct Entry {
    key: String,
    name: String,
}

#[derive(Serialize)]
struct TemplateContext {
    entries: Vec<Entry>,
}

#[get("/")]
fn hello(urls: &State<HashMap<String, WeatherLocation>>) -> Template {
    let mut entries = Vec::new();
    for (key, value) in urls.iter() {
        entries.push(Entry {
            key: key.clone(),
            name: value.display_name.clone(),
        });
    }

    entries.sort_by(|a, b| a.key.cmp(&b.key));

    let context = TemplateContext { entries };
    Template::render("index", &context)
}

#[get("/<name>")]
async fn weather(
    urls: &State<HashMap<String, WeatherLocation>>,
    reqwest_client: &State<ClientWithMiddleware>,
    name: &str,
) -> Result<Template, status::NotFound<String>> {
    let selected_station_url = &urls
        .get(name)
        .ok_or_else(|| status::NotFound("Resource not found.".to_string()))?
        .url;
    let xml = fetch_xml((name, selected_station_url), reqwest_client)
        .await
        .unwrap();
    let weather_data = parse_latest_weather_data(xml.as_ref()).unwrap();

    println!("{:?}", weather_data);

    let context = [
        ("time", &weather_data.time),
        ("wind_speed", &weather_data.wind_speed),
        ("temperature", &weather_data.temperature),
        ("wind_direction", &weather_data.wind_direction),
        ("wind_gust_speed", &weather_data.wind_gust_speed),
        ("name", &urls.get(name).unwrap().display_name),
    ]
    .iter()
    .filter(|(_, value)| value != &"NaN")
    .cloned()
    .collect::<HashMap<_, _>>();

    Ok(Template::render("weather", &context))
}

#[launch]
fn rocket() -> _ {
    let urls = create_station_hashmap();
    let reqwest_client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: HttpCacheOptions::default(),
        }))
        .build();

    rocket::build()
        .manage(urls)
        .manage(reqwest_client)
        .mount("/", routes![hello, weather])
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
}

async fn fetch_xml(
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
#[derive(Debug)]
struct WeatherData {
    time: String,
    wind_speed: String,
    temperature: String,
    wind_direction: String,
    wind_gust_speed: String,
}

fn parse_latest_weather_data(xml: &str) -> Option<WeatherData> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut current_time: Option<String> = None;
    let mut current_wind_speed: Option<String> = None;
    let mut current_temperature: Option<String> = None;
    let mut current_wind_direction: Option<String> = None;
    let mut current_wind_gust_speed: Option<String> = None;
    let mut latest_data: Option<WeatherData> = None;
    let mut current_parameter: Option<ParameterType> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"BsWfs:ParameterName" => {
                    let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                    current_parameter = match contents.as_str() {
                        "WindSpeedMS" => Some(ParameterType::WindSpeed),
                        "Temperature" => Some(ParameterType::Temperature),
                        "WindDirection" => Some(ParameterType::WindDirection),
                        "WindGust" => Some(ParameterType::WindGustSpeed),
                        _ => None,
                    };
                }
                b"BsWfs:ParameterValue" => match current_parameter {
                    Some(ParameterType::WindSpeed) => {
                        let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                        current_wind_speed = Some(contents);
                    }
                    Some(ParameterType::Temperature) => {
                        let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                        current_temperature = Some(contents);
                    }
                    Some(ParameterType::WindDirection) => {
                        let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                        current_wind_direction = Some(contents);
                    }
                    Some(ParameterType::WindGustSpeed) => {
                        let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                        current_wind_gust_speed = Some(contents);
                    }
                    _ => (),
                },
                b"BsWfs:Time" => {
                    let contents: DateTime<Utc> = reader
                        .read_text(e.name(), &mut Vec::new())
                        .unwrap()
                        .parse()
                        .unwrap();
                    let dt_helsinki = contents
                        .with_timezone(&Helsinki)
                        .format("%d/%m/%Y %T")
                        .to_string();
                    current_time = Some(dt_helsinki);
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
        Some(time),
        Some(wind_speed),
        Some(temperature),
        Some(wind_direction),
        Some(wind_gust_speed),
    ) = (
        current_time,
        current_wind_speed,
        current_temperature,
        current_wind_direction,
        current_wind_gust_speed,
    ) {
        latest_data = Some(WeatherData {
            time,
            wind_speed,
            temperature,
            wind_direction,
            wind_gust_speed,
        });
    }

    latest_data
}
