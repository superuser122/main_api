#[macro_use] extern crate rocket;
pub mod models;
pub mod services;
pub mod endpoints;
use dotenv::dotenv;
use rocket::routes;
use std::env;
use endpoints::*;


#[rocket::main]
async fn main() {
    dotenv().ok();


    //Get the databases urls or crush the server
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");

    //Get the databases clients or crash the server  
    let mongo = mongodb::Client::with_uri_str(mongo_url)
                            .await.expect("There was an error parsing mongodb client")
                            .database("userdb");



    let _ = rocket::build()
                .mount("/api", routes![
                    auth_user::login,
                    auth_user::logout,
                    user_crud::read_self,
                    user_crud::delete,
                    user_crud::read_user,
                    user_crud::create,
                    ])
                .manage(mongo)
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
