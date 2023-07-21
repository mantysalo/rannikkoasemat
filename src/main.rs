use std::collections::HashMap;

use chrono::{DateTime, Utc};
use chrono_tz::Europe::Helsinki;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Error;
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

fn create_weather_url(station_id: i32) -> String {
    let mut url = String::from("https://opendata.fmi.fi/wfs?service=WFS&version=2.0.0&request=getFeature&parameters=Temperature,WindDirection,WindSpeedMS,WindGust&storedquery_id=fmi::observations::weather::simple&fmisid=");
    url.push_str(station_id.to_string().as_str());
    url
}

struct WeatherLocation {
    display_name: String,
    url: String,
}

#[get("/<name>")]
async fn weather(
    urls: &State<HashMap<String, WeatherLocation>>,
    name: &str,
) -> Result<Template, status::NotFound<String>> {
    let selected_station_url = &urls
        .get(name)
        .ok_or_else(|| status::NotFound("Resource not found.".to_string()))?
        .url;
    let xml = fetch_xml((name, selected_station_url)).await.unwrap();
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
    let urls = {
        fn create_entry(name: &str, id: i32) -> WeatherLocation {
            WeatherLocation {
                display_name: name.to_string(),
                url: create_weather_url(id),
            }
        }

        let mut map = HashMap::new();
        map.insert(
            "rajakari".to_string(),
            create_entry("Turku Rajakari", 100947),
        );
        map.insert(
            "haapasaari".to_string(),
            create_entry("Kotka Haapasaari", 101042),
        );
        map.insert("rankki".to_string(), create_entry("Kotka Rankki", 101030));
        map.insert(
            "orrengrund".to_string(),
            create_entry("Loviisa Orrengrund", 101039),
        );
        map.insert(
            "kilpilahti_satama".to_string(),
            create_entry("Porvoo Kilpilahti satama", 100683),
        );
        map.insert(
            "emasalo".to_string(),
            create_entry("Porvoo Emäsalo", 101023),
        );
        map.insert(
            "kalbadagrund".to_string(),
            create_entry("Porvoo Kalbådagrund", 101022),
        );
        map.insert(
            "vuosaari_satama".to_string(),
            create_entry("Helsinki Vuosaari satama", 151028),
        );
        map.insert(
            "itatoukki".to_string(),
            create_entry("Sipoo Itätoukki", 105392),
        );
        map.insert(
            "harmaja".to_string(),
            create_entry("Helsinki Harmaja", 100996),
        );
        map.insert(
            "helsinki_majakka".to_string(),
            create_entry("Helsinki Helsingin Majakka", 101003),
        );
        map.insert(
            "makiluoto".to_string(),
            create_entry("Kirkkonummi Mäkiluoto", 100997),
        );
        map.insert(
            "bogaskar".to_string(),
            create_entry("Inkoo Bågaskär", 100969),
        );
        map.insert(
            "jussaro".to_string(),
            create_entry("Raasepori Jussarö", 100965),
        );
        map.insert(
            "tulliniemi".to_string(),
            create_entry("Hanko Tulliniemi", 100946),
        );
        map.insert("russaro".to_string(), create_entry("Hanko Russarö", 100932));
        map.insert("vano".to_string(), create_entry("Kemiönsaari Vänö", 100945));
        map.insert("uto".to_string(), create_entry("Parainen Utö", 100908));
        map.insert("bogskar".to_string(), create_entry("Kökar Bogskär", 100921));
        map.insert(
            "fagerholm".to_string(),
            create_entry("Parainen Fagerholm", 100924),
        );
        map.insert(
            "kirkonkyla".to_string(),
            create_entry("Kumlinge kirkonkylä", 100928),
        );
        map.insert(
            "langnas_satama".to_string(),
            create_entry("Lumparland Långnäs satama", 151048),
        );
        map.insert(
            "lansi_satama".to_string(),
            create_entry("Maarianhamina Länsisatama", 151029),
        );
        map.insert(
            "lotsberget".to_string(),
            create_entry("Maarianhamina Lotsberget", 107383),
        );
        map.insert("nyhamn".to_string(), create_entry("Lemland Nyhamn", 100909));
        map.insert(
            "market".to_string(),
            create_entry("Hammarland Märket", 100919),
        );
        map.insert(
            "isokari".to_string(),
            create_entry("Kustavi Isokari", 101059),
        );
        map.insert(
            "kylmapihlaja".to_string(),
            create_entry("Rauma Kylmäpihlaja", 101061),
        );
        map.insert(
            "tahkoluoto_satama".to_string(),
            create_entry("Pori Tahkoluoto satama", 101267),
        );
        map.insert(
            "kristiinankaupunki_majakka".to_string(),
            create_entry("Kristiinankaupunki Majakka", 101268),
        );
        map.insert(
            "salgrund".to_string(),
            create_entry("Kaskinen Sälgrund", 101256),
        );
        map.insert(
            "bredskaret".to_string(),
            create_entry("Korsnäs Bredskäret", 101479),
        );
        map.insert(
            "strommingsbadan".to_string(),
            create_entry("Maalahti Strömmingsbådan", 101481),
        );
        map.insert(
            "valassaaret".to_string(),
            create_entry("Mustalahti Valassaaret", 101464),
        );
        map.insert(
            "kallan".to_string(),
            create_entry("Pietarsaari Kallan", 101660),
        );
        map.insert("tankar".to_string(), create_entry("Kokkola Tankar", 101661));
        map.insert(
            "ulkokalla".to_string(),
            create_entry("Kalajoki Ulkokalla", 101673),
        );
        map.insert(
            "nahkiainen".to_string(),
            create_entry("Raahe Nahkiainen", 101775),
        );
        map.insert(
            "lapaluoto_satama".to_string(),
            create_entry("Raahe Lapaluoto satama", 101785),
        );
        map.insert(
            "vihresaari_satama".to_string(),
            create_entry("Oulu Vihreäsaari satama", 101794),
        );
        map.insert(
            "marjaniemi".to_string(),
            create_entry("Hailuoto Marjaniemi", 101784),
        );
        map.insert(
            "kemi_majakka".to_string(),
            create_entry("Kemi I Majakka", 101783),
        );
        map.insert("ajos".to_string(), create_entry("Kemi Ajos", 101846));
        map.insert(
            "pulkkilanharju".to_string(),
            create_entry("Asikkala Pulkkilanharju", 101185),
        );
        map.insert(
            "judinsalo".to_string(),
            create_entry("Luhanka Judinsalo", 101362),
        );
        map.insert(
            "hiekkapakka".to_string(),
            create_entry("Lappeenranta Hiekkapakka", 101252),
        );
        map.insert(
            "rukkasluoto".to_string(),
            create_entry("Rantasalmi Rukkasluoto", 101436),
        );
        map.insert(
            "tuiskavanluoto".to_string(),
            create_entry("Liperi Tuiskavanluoto", 101628),
        );
        map.insert(
            "ritoniemi".to_string(),
            create_entry("Kuopio Ritoniemi", 101580),
        );
        map.insert(
            "seitalaassa".to_string(),
            create_entry("Inari Seitalaassa", 129963),
        );
        map
    };
    rocket::build()
        .manage(urls)
        .mount("/", routes![hello, weather])
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
}

async fn fetch_xml((weather_station_location, url): (&str, &str)) -> Result<String, Error> {
    println!("Fetching weather data for {}", weather_station_location);
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
