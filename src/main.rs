#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", routes![index, hello])
        .mount("/home", routes![home_index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "what what"
}

#[get("/template1")]
fn home_index() -> Template {
    Template::render("home/index", context! { field: "value" })
}
