pub mod stations {
    use std::collections::HashMap;

    pub struct WeatherLocation {
        pub display_name: String,
        pub url: String,
    }

    struct WeatherStation {
        id: &'static str,
        display_name: &'static str,
        fmis_id: i32,
    }
    const WEATHER_STATIONS: [WeatherStation; 50] = [
        WeatherStation {
            id: "rajakari",
            display_name: "Turku Rajakari",
            fmis_id: 100947,
        },
        WeatherStation {
            id: "haapasaari",
            display_name: "Kotka Haapasaari",
            fmis_id: 101042,
        },
        WeatherStation {
            id: "rankki",
            display_name: "Kotka Rankki",
            fmis_id: 101030,
        },
        WeatherStation {
            id: "orrengrund",
            display_name: "Loviisa Orrengrund",
            fmis_id: 101039,
        },
        WeatherStation {
            id: "kilpilahti_satama",
            display_name: "Porvoo Kilpilahti satama",
            fmis_id: 100683,
        },
        WeatherStation {
            id: "emasalo",
            display_name: "Porvoo Emäsalo",
            fmis_id: 101023,
        },
        WeatherStation {
            id: "kalbadagrund",
            display_name: "Porvoo Kalbådagrund",
            fmis_id: 101022,
        },
        WeatherStation {
            id: "vuosaari_satama",
            display_name: "Helsinki Vuosaari satama",
            fmis_id: 151028,
        },
        WeatherStation {
            id: "itatoukki",
            display_name: "Sipoo Itätoukki",
            fmis_id: 105392,
        },
        WeatherStation {
            id: "harmaja",
            display_name: "Helsinki Harmaja",
            fmis_id: 100996,
        },
        WeatherStation {
            id: "helsinki_majakka",
            display_name: "Helsinki Helsingin Majakka",
            fmis_id: 101003,
        },
        WeatherStation {
            id: "makiluoto",
            display_name: "Kirkkonummi Mäkiluoto",
            fmis_id: 100997,
        },
        WeatherStation {
            id: "bogaskar",
            display_name: "Inkoo Bågaskär",
            fmis_id: 100969,
        },
        WeatherStation {
            id: "jussaro",
            display_name: "Raasepori Jussarö",
            fmis_id: 100965,
        },
        WeatherStation {
            id: "tulliniemi",
            display_name: "Hanko Tulliniemi",
            fmis_id: 100946,
        },
        WeatherStation {
            id: "russaro",
            display_name: "Hanko Russarö",
            fmis_id: 100932,
        },
        WeatherStation {
            id: "vano",
            display_name: "Kemiönsaari Vänö",
            fmis_id: 100945,
        },
        WeatherStation {
            id: "uto",
            display_name: "Parainen Utö",
            fmis_id: 100908,
        },
        WeatherStation {
            id: "bogskar",
            display_name: "Kökar Bogskär",
            fmis_id: 100921,
        },
        WeatherStation {
            id: "fagerholm",
            display_name: "Parainen Fagerholm",
            fmis_id: 100924,
        },
        WeatherStation {
            id: "kirkonkyla",
            display_name: "Kumlinge kirkonkylä",
            fmis_id: 100928,
        },
        WeatherStation {
            id: "langnas_satama",
            display_name: "Lumparland Långnäs satama",
            fmis_id: 151048,
        },
        WeatherStation {
            id: "lansi_satama",
            display_name: "Maarianhamina Länsisatama",
            fmis_id: 151029,
        },
        WeatherStation {
            id: "lotsberget",
            display_name: "Maarianhamina Lotsberget",
            fmis_id: 107383,
        },
        WeatherStation {
            id: "nyhamn",
            display_name: "Lemland Nyhamn",
            fmis_id: 100909,
        },
        WeatherStation {
            id: "market",
            display_name: "Hammarland Märket",
            fmis_id: 100919,
        },
        WeatherStation {
            id: "isokari",
            display_name: "Kustavi Isokari",
            fmis_id: 101059,
        },
        WeatherStation {
            id: "kylmapihlaja",
            display_name: "Rauma Kylmäpihlaja",
            fmis_id: 101061,
        },
        WeatherStation {
            id: "tahkoluoto_satama",
            display_name: "Pori Tahkoluoto satama",
            fmis_id: 101267,
        },
        WeatherStation {
            id: "kristiinankaupunki_majakka",
            display_name: "Kristiinankaupunki Majakka",
            fmis_id: 101268,
        },
        WeatherStation {
            id: "salgrund",
            display_name: "Kaskinen Sälgrund",
            fmis_id: 101256,
        },
        WeatherStation {
            id: "bredskaret",
            display_name: "Korsnäs Bredskäret",
            fmis_id: 101479,
        },
        WeatherStation {
            id: "strommingsbadan",
            display_name: "Maalahti Strömmingsbådan",
            fmis_id: 101481,
        },
        WeatherStation {
            id: "valassaaret",
            display_name: "Mustalahti Valassaaret",
            fmis_id: 101464,
        },
        WeatherStation {
            id: "kallan",
            display_name: "Pietarsaari Kallan",
            fmis_id: 101660,
        },
        WeatherStation {
            id: "tankar",
            display_name: "Kokkola Tankar",
            fmis_id: 101661,
        },
        WeatherStation {
            id: "ulkokalla",
            display_name: "Kalajoki Ulkokalla",
            fmis_id: 101673,
        },
        WeatherStation {
            id: "nahkiainen",
            display_name: "Raahe Nahkiainen",
            fmis_id: 101775,
        },
        WeatherStation {
            id: "lapaluoto_satama",
            display_name: "Raahe Lapaluoto satama",
            fmis_id: 101785,
        },
        WeatherStation {
            id: "vihresaari_satama",
            display_name: "Oulu Vihreäsaari satama",
            fmis_id: 101794,
        },
        WeatherStation {
            id: "marjaniemi",
            display_name: "Hailuoto Marjaniemi",
            fmis_id: 101784,
        },
        WeatherStation {
            id: "kemi_majakka",
            display_name: "Kemi I Majakka",
            fmis_id: 101783,
        },
        WeatherStation {
            id: "ajos",
            display_name: "Kemi Ajos",
            fmis_id: 101846,
        },
        WeatherStation {
            id: "pulkkilanharju",
            display_name: "Asikkala Pulkkilanharju",
            fmis_id: 101185,
        },
        WeatherStation {
            id: "judinsalo",
            display_name: "Luhanka Judinsalo",
            fmis_id: 101362,
        },
        WeatherStation {
            id: "hiekkapakka",
            display_name: "Lappeenranta Hiekkapakka",
            fmis_id: 101252,
        },
        WeatherStation {
            id: "rukkasluoto",
            display_name: "Rantasalmi Rukkasluoto",
            fmis_id: 101436,
        },
        WeatherStation {
            id: "tuiskavanluoto",
            display_name: "Liperi Tuiskavanluoto",
            fmis_id: 101628,
        },
        WeatherStation {
            id: "ritoniemi",
            display_name: "Kuopio Ritoniemi",
            fmis_id: 101580,
        },
        WeatherStation {
            id: "seitalaassa",
            display_name: "Inari Seitalaassa",
            fmis_id: 129963,
        },
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
                .map(
                    |WeatherStation {
                         display_name,
                         fmis_id,
                         id,
                     }| (id.to_string(), create_entry(display_name, *fmis_id)),
                )
                .collect()
        };
        urls
    }
    fn create_weather_url(station_id: i32) -> String {
        format!("https://opendata.fmi.fi/wfs?service=WFS&version=2.0.0&request=getFeature&parameters=Temperature,WindDirection,WindSpeedMS,WindGust&storedquery_id=fmi::observations::weather::simple&fmisid={}", station_id)
    }
}
