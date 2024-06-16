#[macro_use]
extern crate rocket;
mod fmi_api;
mod responder;
mod routes;
mod stations;
mod utils;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use stations::stations::{create_station_hashmap, WeatherLocation};
#[launch]
pub fn rocket() -> _ {
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
        .mount(
            "/",
            routes![routes::index::home_page, routes::weather::get_weather],
        )
        .attach(Template::fairing())
        .mount("/static", FileServer::from("./static"))
}
