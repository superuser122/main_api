use crate::models::{User, LoginUser, UserSession};
use mongodb::{bson::{doc, oid::ObjectId}, Client,};
use bcrypt::{DEFAULT_COST, hash, verify};

pub async fn login_service(database :&mut mongodb::Database, login_user: LoginUser)-> Result<String, String>{
    let user = get_user(database, &login_user.user_name).await?;
    let pwd_hsh = hash(&login_user.password, DEFAULT_COST).map_err(|e| e.to_string())?; 
    if !verify(&user.password, &pwd_hsh).map_err(|e| e.to_string())?{
        return Err("Invalid username or password".to_string());
    }


    todo!();
}

async fn get_user(database :&mut  mongodb::Database, user_name: &String)-> Result<User, String>{
    let user_collection = database.collection::<User>("users");
    let filter = doc!{"user_name": user_name};
    let user = user_collection.find_one(filter, None).await.map_err(|e| e.to_string())?;
    match user{
        Some(user) => Ok(user),
        None => Err("Invalid username or password".to_string())
    }
}

async fn create_session() -> Result<String, String>{
    todo!();
}

async fn delete_session()-> Result<(), String>{
    todo!();
}

async fn get_session()-> Result<Option<UserSession>, String>{
    todo!();
}


//Unit testing

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[async_test]
    async fn get_user_test() {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").unwrap();
        let mut mongo = mongodb::Client::with_uri_str(mongo_url)
                            .await.unwrap()
                            .database("userdb");
        let user_name = "vasilis".to_string();
        let user = get_user(&mut mongo, &user_name).await;
        assert!(user.is_ok());
    }
}



