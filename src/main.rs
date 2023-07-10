use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match fetch_xml().await {
        Ok(xml) => {
            let weather_data = parse_latest_weather_data(&xml);

            match weather_data {
                Some(data) => println!(
                    "Wind speed: {} m/s\nWind gust: {} m/s\nWind direction: {}\u{00B0}\nTemperature: {}\u{00B0}C\nTime: {}",
                    data.wind_speed, data.wind_gust_speed, data.wind_direction, data.temperature, data.time
                ),
                None => println!("Wind speed not found"),
            }

            Ok(())
        }
        Err(e) => {
            println!(
                "Error fetching XML: {}",
                match e.url() {
                    None => e.to_string(),
                    Some(url) => url.to_string(),
                }
            );
            Err(e)
        }
    }
}

async fn fetch_xml() -> Result<String, Error> {
    let url = "https://opendata.fmi.fi/wfs?service=WFS&version=2.0.0&request=getFeature&parameters=Temperature,WindDirection,WindSpeedMS,WindGust&storedquery_id=fmi::observations::weather::simple&fmisid=100947";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    Ok(body)
}
enum ParameterType {
    WindSpeed,
    Temperature,
    WindDirection,
    WindGustSpeed,
}
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
                    let contents = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                    current_time = Some(contents);
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
