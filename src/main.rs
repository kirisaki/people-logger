#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

#[derive(Debug, Deserialize, Serialize)]
struct Actcast {
    count: u64,
    timestamp: f64,
    device_name: String,
}

#[post("/actcast", format = "application/json", data = "<cast>")]
fn actcast(cast: Json<Actcast>) -> String {
    cast.count.to_string()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![actcast, index]).launch();
}
