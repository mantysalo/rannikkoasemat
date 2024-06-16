use crate::stations::stations::WeatherLocation;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;

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
pub fn home_page(urls: &State<HashMap<String, WeatherLocation>>) -> Template {
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
