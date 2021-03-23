use diesel::sql_types::Timestamp;
use super::schema::counts;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Count {
    pub id: u64,
    pub device_name: String,
    pub num_of_people: i64,
    pub recorded_at: Timestamp,
}

#[derive(Insertable)]
#[table_name="counts"]
pub struct NewCount<'a> {
    pub device_name: &'a str,
    pub num_of_people: i32,
    pub recorded_at: NaiveDateTime,
}
