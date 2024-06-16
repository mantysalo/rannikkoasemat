use crate::fmi_api::{fetch_xml, parse_latest_weather_data};
use crate::responder::WeatherResponse;
use crate::WeatherLocation;
use reqwest_middleware::ClientWithMiddleware;
use rocket::State;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/<name>")]
pub async fn get_weather(
    urls: &State<HashMap<String, WeatherLocation>>,
    reqwest_client: &State<ClientWithMiddleware>,
    name: &str,
) -> WeatherResponse {
    let selected_station = match urls.get(name) {
        Some(location) => location,
        None => {
            return WeatherResponse::NotFound("Resource not found.".to_string());
        }
    };
    let selected_station_url = selected_station.url.as_str();
    let xml = match fetch_xml((name, selected_station_url), reqwest_client).await {
        Ok(xml) => xml,
        Err(e) => {
            println!("Failed to fetch XML: {}", e);
            return WeatherResponse::InternalServerError(
                "Failed to fetch weather data.".to_string(),
            );
        }
    };

    let weather_data = match parse_latest_weather_data(xml.as_ref()) {
        Some(data) => data,
        None => {
            println!("Failed to parse weather data");
            return WeatherResponse::InternalServerError(
                "Failed to parse weather data.".to_string(),
            );
        }
    };

    println!("{:?}", weather_data);

    let context = [
        ("time", &weather_data.time_of_observation),
        ("wind_speed", &weather_data.wind_speed_meters_per_second),
        ("temperature", &weather_data.temperature_celsius),
        ("wind_direction", &weather_data.wind_direction),
        (
            "wind_gust_speed",
            &weather_data.wind_gust_speed_meters_per_second,
        ),
        ("name", &selected_station.display_name),
    ]
    .iter()
    .filter(|(_, value)| value != &"NaN")
    .cloned()
    .collect::<HashMap<_, _>>();

    WeatherResponse::Template(Template::render("weather", &context), weather_data)
}
