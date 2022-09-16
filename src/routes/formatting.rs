use rocket_dyn_templates::{Template, context};
use rocket::http::uri::Origin;

#[get("/")]
pub fn index(origin: &Origin) -> Template {
    let version = option_env!("VERSION").unwrap_or("no_version").to_string();

    Template::render(
    "formatting/index", 
    context! { 
        version: version,
        origin: origin 
    })
}