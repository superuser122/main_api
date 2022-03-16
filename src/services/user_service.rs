use crate::models::{user::{User, LoginUser}, sessions::{UserSession, SessionId}};
use bson::Document;
use mongodb::{bson::{doc, oid::ObjectId,}, Collection};
use bcrypt::{hash, verify};
use std::str::FromStr;

use crate::services::session_service::*;

pub async fn login_service(database : &mongodb::Database, login_user: LoginUser)-> Result<String, String>{
    let user = get_user(database, &login_user.user_name).await?;
    if !verify( &login_user.password,&user.password).map_err(|e| e.to_string())?{
        return Err("Invalid username or password".to_string());
    }
    create_session(&user, database).await
}

pub async fn logout_service(database : &mongodb::Database, session_id : String) -> Result<(), String>{
    let session_collection  : Collection<Document>    = database.collection("sessions");
    validate_session(&session_id, database).await.map_err(|e| e.to_string())?;
    delete_session(&session_id, &session_collection).await.map_err(|e| e.to_string())?;
    Ok(())
}

//Get user from mongo database by name
pub async fn get_user(database : &mongodb::Database, user_name: &String)-> Result<User, String>{
    let user_collection = database.collection::<User>("users");
    let filter = doc!{"user_name": user_name};
    let user = user_collection.find_one(filter, None).await.map_err(|e| e.to_string())?;
    match user{
        Some(user) => Ok(user),
        None => Err("Invalid username or password".to_string())
    }
}

pub async fn create_user(database : &mongodb::Database, mut user: User) -> Result<(), String>{
    user.password = hash(user.password, 4).map_err(|e| e.to_string())?;
    let user_collection = database.collection("users");
    let user_bson = bson::to_bson(&user).map_err(|e| e.to_string())?;
    let user_doc = match user_bson.as_document(){
        Some(doc) => doc,
        None => return Err("Unable to parse user doc!".to_string())
    };
    user_collection.insert_one(user_doc.to_owned(), None).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_user(database : &mongodb::Database, user_id: String) -> Result<(), String>{
    let user_collection : Collection<Document> = database.collection("users");
    let obj_id = ObjectId::from_str(user_id.as_str()).map_err(|e| e.to_string())?;
    let filter = doc!{"_id": obj_id};
    user_collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn update_user(database : &mongodb::Database, mut user: User) -> Result<(), String>{
    user.password = hash(user.password, 4).map_err(|e| e.to_string())?;
    let filter = doc!{"_id": user.id};
    let user_collection : Collection<Document> = database.collection("users");
    let user_bson = bson::to_bson(&user).map_err(|e| e.to_string())?;
    let user_doc = match user_bson.as_document(){
        Some(doc) => doc,
        None => return Err("Unable to parse user doc!".to_string())
    };
    user_collection.find_one_and_replace(filter, user_doc, None).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_users_num(database : &mongodb::Database, db_name: String)-> Result<u8, String>{
    let user_collection : Collection<Document> = database.collection("users");
    let filter = doc!{"database": db_name};
    let doc_num = user_collection.count_documents(filter, None).await.map_err(|e| e.to_string())?;
    Ok(doc_num as u8)
}

//Unit testing

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use crate::models::{user::{User, LoginUser, UserRole, System}, sessions::{UserSession, SessionId}};


    
    async fn get_mongo() -> mongodb::Database { 
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").unwrap();
        mongodb::Client::with_uri_str(mongo_url)
                            .await.unwrap()
                            .database("userdb")
    }

    #[async_test]
    async fn create_user_test() {
        let mongo = get_mongo().await;
        let user = User {
            id: None,
            user_name: "vasilis".to_string(),
            password: "strongpassowrd".to_string(),
            email: "vasileiosnl@gmail.com".to_string(),
            role : UserRole::Admin,
            max_users: None,
            system: vec![ System::Invoicing ],
            database: "userdb".to_string(),
            expiration_dt: chrono::MAX_DATETIME,
        };
        let res = create_user(&mongo, user).await;

        
        assert!(res.is_ok());

    }


    #[async_test]
    async fn get_user_test() {
        let mut mongo = get_mongo().await;
        let user_name = "vasilis".to_string();
        let user = get_user(&mut mongo, &user_name).await;
        assert!(user.is_ok());
        if let Ok(user) = user{ 
            println!("{:?}" , user);

        }
    }


}