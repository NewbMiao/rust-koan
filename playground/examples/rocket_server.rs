#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Hello<'a> {
    name: &'a str,
}

#[get("/", format = "html")]
fn hello() -> Json<Hello<'static>> {
    Json(Hello { name: "Tyr" })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
