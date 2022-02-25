#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
                .mount("/api", routes![world])
                .launch()
                .await;
}
