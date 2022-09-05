#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template};
use rocket::fs::{FileServer, relative};
use rocket::Request;

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", routes![routes::home::index])
        .mount("/formatting", routes![routes::formatting::index])
        .mount("/timezone", routes![routes::timezone::index])
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}
