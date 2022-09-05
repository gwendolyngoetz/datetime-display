use rocket::http::uri::Origin;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(origin: &Origin) -> Template {
    Template::render(
        "home/index",
        context! {
            field: "value",
            origin: origin
        },
    )
}
