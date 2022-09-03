use rocket_dyn_templates::{Template, context};

#[get("/index")]
pub fn index() -> Template {
    Template::render("home/index", context! { field: "value" })
}