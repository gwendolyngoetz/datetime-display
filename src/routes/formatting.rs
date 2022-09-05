use rocket_dyn_templates::{Template, context};
use rocket::http::uri::Origin;

#[get("/")]
pub fn index(origin: &Origin) -> Template {
    Template::render("formatting/index", context! { origin: origin })
}