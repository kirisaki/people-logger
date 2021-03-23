#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::RunQueryDsl;
use rocket::response::content;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel::PgConnection;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

use models::NewCount;
use models::Count;

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

#[derive(Serialize)]
struct DataResponse {
    data: Vec<Data>,
}

#[derive(Serialize)]
struct Data {
    num_of_people: i32,
    msec: i64,
}

#[post("/actcast", format = "application/json", data = "<cast>")]
fn actcast(conn: DBConn, cast: Json<Incoming>) -> status::Accepted<content::Json<&'static str>> {
    use schema::counts::dsl::*;

    let incoming = cast.into_inner();
    let secs = incoming.timestamp as i64 / 1000;
    let nsecs = (incoming.timestamp as i64 % 1000) as u32 * 1000_000;
    let new_count = NewCount {
        device_name: incoming.device_name,
        num_of_people: incoming.num_of_people,
        recorded_at: NaiveDateTime::from_timestamp(secs, nsecs),
    };
    insert_into(counts).values(new_count).execute(&*conn).expect("Failed to insert");
    status::Accepted(Some(content::Json("{\"message\":\"success\"}")))
}

#[get("/device/<name>/data")]
fn get_device_data(conn: DBConn, name: String) -> Result<Json<DataResponse>, status::NotFound<content::Json<&'static str>>> {
    use schema::counts::dsl::*;

    let data = counts.filter(device_name.eq(name)).limit(100).load::<Count>(&*conn).expect("Error loading counts");
    if data.len() == 0 {
        Err(status::NotFound(content::Json("{\"message\":\"Device not found\"}")))
    } else {
        Ok(Json(DataResponse{data: data.into_iter().map(|v| Data{num_of_people: v.num_of_people, msec: v.recorded_at.timestamp_millis()}).collect()}))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    rocket::ignite()
        .attach(DBConn::fairing())
        .mount("/api", routes![get_device_data, actcast, index])
        .launch();
}
