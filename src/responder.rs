use crate::fmi_api::WeatherData;
use rocket::response::{Responder, Result};
use rocket::{
    http::Status,
    response::{content, status},
    Request,
};
use rocket_dyn_templates::Template;

#[derive(Debug)]
pub enum WeatherResponse {
    Template(Template, WeatherData),
    NotFound(String),
    InternalServerError(String),
}

impl<'r> Responder<'r, 'static> for WeatherResponse {
    fn respond_to(self, req: &Request<'_>) -> Result<'static> {
        let accept_json = req
            .headers()
            .get_one("Accept")
            .map_or(false, |accept| accept == "application/json");

        match self {
            WeatherResponse::Template(template, weather_data) => {
                if accept_json {
                    let json = rocket::serde::json::Json(weather_data);
                    json.respond_to(req)
                } else {
                    template.respond_to(req)
                }
            }
            WeatherResponse::NotFound(message) => {
                if accept_json {
                    let json = content::RawJson(format!("{{ \"error\": \"{}\" }}", message));
                    status::Custom(Status::NotFound, json).respond_to(req)
                } else {
                    status::Custom(Status::NotFound, message).respond_to(req)
                }
            }
            WeatherResponse::InternalServerError(message) => {
                if accept_json {
                    let json = content::RawJson(format!("{{ \"error\": \"{}\" }}", message));
                    status::Custom(Status::InternalServerError, json).respond_to(req)
                } else {
                    status::Custom(Status::InternalServerError, message).respond_to(req)
                }
            }
        }
    }
}
