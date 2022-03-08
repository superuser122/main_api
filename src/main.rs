#[macro_use] extern crate rocket;
pub mod models;
pub mod services;
pub mod databases;
pub mod endpoints;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
                .mount("/api", routes![endpoints::world])
                .launch()
                .await;
}
