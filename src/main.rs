#[macro_use] extern crate rocket;
pub mod models;
pub mod services;
pub mod databases;
pub mod endpoints;
use dotenv::dotenv;
use std::env;
use models::DbClients;


#[rocket::main]
async fn main() {
    dotenv().ok();


    //Get the databases urls or crush the server
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    //Get the databases clients or crash the server  
    let redis = redis::Client::open(redis_url).expect("There was an error parsing redis client");

    //If the clients are ok we put them in a state to use them across endpoints 
    let db_state = DbClients {redis};

    let _ = rocket::build()
                .mount("/api", routes![endpoints::world])
                .manage(db_state)
                .launch()
                .await;
}


//Unit testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_redis_url() {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL");

        assert!(redis_url.is_ok());

    }


    #[test]
    fn test_redis_client() {
        dotenv().ok();
        let redis = redis::Client::open(env::var("REDIS_URL").unwrap());

        assert!(redis.is_ok());
    }
}
