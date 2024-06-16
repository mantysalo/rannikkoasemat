pub mod stations {
    use std::collections::HashMap;

    pub struct WeatherLocation {
        pub display_name: String,
        pub url: String,
    }
    const WEATHER_STATIONS: [(&str, &str, i32); 50] = [
        ("rajakari", "Turku Rajakari", 100947),
        ("haapasaari", "Kotka Haapasaari", 101042),
        ("rankki", "Kotka Rankki", 101030),
        ("orrengrund", "Loviisa Orrengrund", 101039),
        ("kilpilahti_satama", "Porvoo Kilpilahti satama", 100683),
        ("emasalo", "Porvoo Emäsalo", 101023),
        ("kalbadagrund", "Porvoo Kalbådagrund", 101022),
        ("vuosaari_satama", "Helsinki Vuosaari satama", 151028),
        ("itatoukki", "Sipoo Itätoukki", 105392),
        ("harmaja", "Helsinki Harmaja", 100996),
        ("helsinki_majakka", "Helsinki Helsingin Majakka", 101003),
        ("makiluoto", "Kirkkonummi Mäkiluoto", 100997),
        ("bogaskar", "Inkoo Bågaskär", 100969),
        ("jussaro", "Raasepori Jussarö", 100965),
        ("tulliniemi", "Hanko Tulliniemi", 100946),
        ("russaro", "Hanko Russarö", 100932),
        ("vano", "Kemiönsaari Vänö", 100945),
        ("uto", "Parainen Utö", 100908),
        ("bogskar", "Kökar Bogskär", 100921),
        ("fagerholm", "Parainen Fagerholm", 100924),
        ("kirkonkyla", "Kumlinge kirkonkylä", 100928),
        ("langnas_satama", "Lumparland Långnäs satama", 151048),
        ("lansi_satama", "Maarianhamina Länsisatama", 151029),
        ("lotsberget", "Maarianhamina Lotsberget", 107383),
        ("nyhamn", "Lemland Nyhamn", 100909),
        ("market", "Hammarland Märket", 100919),
        ("isokari", "Kustavi Isokari", 101059),
        ("kylmapihlaja", "Rauma Kylmäpihlaja", 101061),
        ("tahkoluoto_satama", "Pori Tahkoluoto satama", 101267),
        (
            "kristiinankaupunki_majakka",
            "Kristiinankaupunki Majakka",
            101268,
        ),
        ("salgrund", "Kaskinen Sälgrund", 101256),
        ("bredskaret", "Korsnäs Bredskäret", 101479),
        ("strommingsbadan", "Maalahti Strömmingsbådan", 101481),
        ("valassaaret", "Mustalahti Valassaaret", 101464),
        ("kallan", "Pietarsaari Kallan", 101660),
        ("tankar", "Kokkola Tankar", 101661),
        ("ulkokalla", "Kalajoki Ulkokalla", 101673),
        ("nahkiainen", "Raahe Nahkiainen", 101775),
        ("lapaluoto_satama", "Raahe Lapaluoto satama", 101785),
        ("vihresaari_satama", "Oulu Vihreäsaari satama", 101794),
        ("marjaniemi", "Hailuoto Marjaniemi", 101784),
        ("kemi_majakka", "Kemi I Majakka", 101783),
        ("ajos", "Kemi Ajos", 101846),
        ("pulkkilanharju", "Asikkala Pulkkilanharju", 101185),
        ("judinsalo", "Luhanka Judinsalo", 101362),
        ("hiekkapakka", "Lappeenranta Hiekkapakka", 101252),
        ("rukkasluoto", "Rantasalmi Rukkasluoto", 101436),
        ("tuiskavanluoto", "Liperi Tuiskavanluoto", 101628),
        ("ritoniemi", "Kuopio Ritoniemi", 101580),
        ("seitalaassa", "Inari Seitalaassa", 129963),
    ];
    pub fn create_station_hashmap() -> HashMap<String, WeatherLocation> {
        let urls = {
            fn create_entry(name: &str, id: i32) -> WeatherLocation {
                WeatherLocation {
                    display_name: name.to_string(),
                    url: create_weather_url(id),
                }
            }
            WEATHER_STATIONS
                .iter()
                .map(|(key, name, id)| (key.to_string(), create_entry(name, *id)))
                .collect()
        };
        urls
    }
    fn create_weather_url(station_id: i32) -> String {
        format!("https://opendata.fmi.fi/wfs?service=WFS&version=2.0.0&request=getFeature&parameters=Temperature,WindDirection,WindSpeedMS,WindGust&storedquery_id=fmi::observations::weather::simple&fmisid={}", station_id)
    }
}
