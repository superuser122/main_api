use crate::models::{User, LoginUser, UserSession, DbClients};
use bson::Document;
use mongodb::{bson::{doc, oid::ObjectId,}, Client,Collection};
use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::State;
use chrono::Utc;


pub async fn login_service(state: &State<DbClients>, login_user: LoginUser)-> Result<String, String>{
    let user = get_user(&state.mongo, &login_user.user_name).await?;
    if !verify( &login_user.password,&user.password).map_err(|e| e.to_string())?{
        return Err("Invalid username or password".to_string());
    }
    create_session(&user, &state.mongo).await
}

//Get user from mongo database
async fn get_user(database : &mongodb::Database, user_name: &String)-> Result<User, String>{
    let user_collection = database.collection::<User>("users");
    let filter = doc!{"user_name": user_name};
    let user = user_collection.find_one(filter, None).await.map_err(|e| e.to_string())?;
    match user{
        Some(user) => Ok(user),
        None => Err("Invalid username or password".to_string())
    }
}

async fn create_session(user: &User, database : &mongodb::Database,) -> Result<String, String>{
    let session_collection = database.collection("sessions");
    let session = UserSession{
        id: None,
        user_id: user.id.unwrap().to_string(),
        database: user.database.clone(),
        system: user.system.to_owned(),
        dt: Utc::now()
    };
    let session_ser = bson::to_bson(&session).map_err(|e| e.to_string())?;
    let session_doc = match session_ser.as_document(){
        Some(doc) => doc,
        None => return Err("Unable to parse doc!".to_string())
    };
    let user_id = match user.id {
        Some(id) => id.to_string(),
        None => return Err(String::from("User id is not set"))
    };
    let _ = delete_session(user_id, &session_collection).await?;
    let ins_id =session_collection.insert_one(session_doc.to_owned(),  None).await.map_err(|e| e.to_string())?;
    Ok(ins_id.inserted_id.as_object_id().unwrap().to_string())

}

async fn delete_session(user_id: String, session_collection: &Collection<Document>)-> Result<(), String>{
    let filter = doc!{"user_id": user_id};
    let _ = session_collection.delete_many(filter, None).await.map_err(|e| e.to_string())?;
    Ok(())
    
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
    use mongodb::{bson::{doc, oid::ObjectId,}, Client,};
    use std::str::FromStr;
    
    async fn get_mongo() -> mongodb::Database { 
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").unwrap();
        mongodb::Client::with_uri_str(mongo_url)
                            .await.unwrap()
                            .database("userdb")
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
    #[async_test]
    async fn create_session_test(){
        let mongo = get_mongo().await;
        let user = User {
            id: Some(ObjectId::from_str("6229fa135e61ec4dd16fc396").unwrap()),
            user_name: "vasilis".to_string(),
            password: "sdjhfsdjkhjfdh".to_string(),
            email: "vasileiosnl@gmail.com".to_string(),
            role : crate::models::UserRole::Admin,
            system: vec![ crate::models::System::Invoicing ],
            database: "userdb".to_string()
        };
        let session_str = create_session(&user, &mongo).await.unwrap();
        println!("{:?}", session_str);


    }
}



