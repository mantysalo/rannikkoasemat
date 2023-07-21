pub mod stations {
    use std::collections::HashMap;

    pub struct WeatherLocation {
        pub display_name: String,
        pub url: String,
    }

    pub fn create_station_hashmap() -> HashMap<String, WeatherLocation> {
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
        urls
    }
    fn create_weather_url(station_id: i32) -> String {
        let mut url = String::from("https://opendata.fmi.fi/wfs?service=WFS&version=2.0.0&request=getFeature&parameters=Temperature,WindDirection,WindSpeedMS,WindGust&storedquery_id=fmi::observations::weather::simple&fmisid=");
        url.push_str(station_id.to_string().as_str());
        url
    }
}
