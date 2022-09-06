use chrono::{Local};

// use chrono::offset::{Local, Utc};
use rocket::{http::uri::Origin};
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(origin: &Origin) -> Template {
    let localdatetime = Local::now();
    let localdate = localdatetime.format("%-m/%-d/%Y").to_string();
    let localtime = localdatetime.format("%-I:%M:%S %p").to_string();
    let localtimezone = localdatetime.format("%Z").to_string();

    let utcdatetime = localdatetime.naive_utc();
    let utcdate = utcdatetime.format("%-m/%-d/%Y").to_string();
    let utctime = utcdatetime.format("%-I:%M:%S %p").to_string();
    let utctimezone = "00:00";
    
    Template::render(
        "home/index",
        context! {
            origin: origin,
            localdate: localdate,
            localtime: localtime,
            localtimezone: localtimezone,
            utcdate: utcdate,
            utctime: utctime,
            utctimezone: utctimezone
        }
    )
}
