use rocket_dyn_templates::{Template, context};
use rocket::http::uri::Origin;

#[get("/")]
pub fn index(origin: &Origin) -> Template {
    let version = match std::env::var("VERSION") {
        Ok(value) => value,
        Err(_e) => String::from("no_version")
    };
    
    Template::render(
    "timezone/index", 
    context! { 
        version: version,
        origin: origin 
    })
}