use crate::models::{User, LoginUser};
use mongodb::{bson::{doc, oid::ObjectId}, Client,};

pub async fn login_service(database : mongodb::Database, user: LoginUser)-> Result<String, String>{
    
    todo!();
}

async fn get_user(database : mongodb::Database, user_name: String)-> Result<User, String>{
    let user_collection = database.collection::<User>("users");
    let filter = doc!{"user_name": user_name};
    let user = user_collection.find_one(filter, None).await.map_err(|e| e.to_string())?;
    match user{
        Some(user) => Ok(user),
        None => Err("User name not found".to_string())
    }
}



//Unit testing

#[cfg(test)]
mod tests {
    use super::*;

    #[async_test]
    async fn get_user_test() {
        assert!(true);
    }



}



