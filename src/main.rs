#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template};
use rocket::fs::{FileServer, relative};

mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/home", routes![
            routes::home::index
        ])
}



/*
TODO

Unix Epoch
    To Date
    From Date
Time Zone
    Convert to Different Time Zone
    Show Local Time
    Show UTC
Formatting
    Cheat Sheet of Format Codes
    Format Preview/Builder
    Convert Between Formats
    Parse Format From Date
    Examples
Date Diff
Human Readable 

Links

https://devhints.io/datetime

*/