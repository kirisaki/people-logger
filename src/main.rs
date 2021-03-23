#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use diesel::insert_into;
use diesel::RunQueryDsl;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::PgConnection;
use serde::{Deserialize};
use chrono::NaiveDateTime;

use models::NewCount;

pub mod schema;
pub mod models;

#[database("postgres_log")]
struct DBConn(PgConnection);

#[derive(Deserialize)]
struct Incoming<'a> {
    device_name: &'a str,
    num_of_people: i32,
    timestamp: f64,
}

#[post("/actcast", format = "application/json", data = "<cast>")]
fn actcast(conn: DBConn, cast: Json<Incoming>) -> String {
    use schema::counts::dsl::*;

    let incoming = cast.into_inner();
    let secs = incoming.timestamp as i64 / 1000;
    let nsecs = (incoming.timestamp as i64 - secs * 1000) as u32 * 1000_000;
    println!("{:?}", nsecs);
    let new_count = NewCount {
        device_name: incoming.device_name,
        num_of_people: incoming.num_of_people,
        recorded_at: NaiveDateTime::from_timestamp(secs, nsecs),
    };
    insert_into(counts).values(new_count).execute(&*conn).expect("Failed to insert");
    "{\"message\":\"success\"}".to_string()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    rocket::ignite()
        .attach(DBConn::fairing())
        .mount("/", routes![actcast, index])
        .launch();
}
