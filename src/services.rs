use crate::models::{User, LoginUser, UserSession, DbClients};
use bson::Document;
use mongodb::{bson::{doc, oid::ObjectId,}, Collection};
use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::State;
use chrono::{Utc, Duration};
use std::str::FromStr;


pub async fn login_service(state: &State<DbClients>, login_user: LoginUser)-> Result<String, String>{
    let user = get_user(&state.mongo, &login_user.user_name).await?;
    if !verify( &login_user.password,&user.password).map_err(|e| e.to_string())?{
        return Err("Invalid username or password".to_string());
    }
    create_session(&user, &state.mongo).await
}

pub async fn logout_service(state: &State<DbClients>, session_id : String) -> Result<(), String>{
    let session_collection  : Collection<Document>    = state.mongo.collection("sessions");
    validate_session(&session_id, &state.mongo).await.map_err(|e| e.to_string())?;
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

async fn create_session(user: &User, database : &mongodb::Database,) -> Result<String, String>{
    if user.expiration_dt < Utc::now(){
        return Err("Subscription has expired".to_string());
    }
    let session_collection = database.collection("sessions");
    let session = UserSession{
        id: None,
        user: user.clone(),
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
    let _ = delete_sessions(user_id, &session_collection).await?;
    let ins_id =session_collection.insert_one(session_doc.to_owned(),  None).await.map_err(|e| e.to_string())?;
    Ok(ins_id.inserted_id.as_object_id().unwrap().to_string())

}

pub async fn delete_sessions(user_id: String, session_collection: &Collection<Document>)-> Result<(), String>{
    let filter = doc!{"user": { "_id" : user_id}};
    let _ = session_collection.delete_many(filter, None).await.map_err(|e| e.to_string())?;
    Ok(())
    
}

pub async fn delete_session(session_id: &String, session_collection: &Collection<Document>)-> Result<(), String>{
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|e| e.to_string())?;
    let filter = doc!{"_id": obj_id};
    let _ = session_collection.delete_one(filter, None).await.map_err(|e| e.to_string())?;
    Ok(())

}

pub async fn get_session(session_id: &String, database : &mongodb::Database)-> Result<Option<UserSession>, String>{
    let session_collection = database.collection::<UserSession>("sessions");
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|e| e.to_string())?;
    let session = session_collection.find_one(doc! {"_id" : obj_id },None).await.map_err(|e| e.to_string())?;
    Ok(session)
}

pub async fn update_session_dt(session_id: String, database : &mongodb::Database)-> Result<(), String>{
    let session_collection = database.collection::<UserSession>("sessions");
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|e| e.to_string())?;
    let filter =doc!{"_id" : obj_id };
    let update = doc!{ "$currentDate" : {"dt" : true}};
    session_collection.update_one(filter, update, None).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn validate_session(session_id: &String, database : &mongodb::Database) -> Result<UserSession, String> {
    let session_opt = get_session(&session_id, database).await.map_err(|e| e.to_string())?;
    let session = match session_opt {
        Some(s) => s,
        None => return Err("No session found".to_string())
    };
    let valid_till = session.dt + Duration::hours(1);
    if valid_till < Utc::now() {
        return Err("Session has expired".to_string());
    }
    if session.user.expiration_dt < Utc::now() {
        return Err("Subscription has expired".to_string());
    }
    update_session_dt(session_id.clone(), database).await.map_err(|e| e.to_string())?;
    Ok(session)
}


//Unit testing

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use mongodb::bson::oid::ObjectId;
    use std::str::FromStr;
    
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
            email: "vasileiosnl@gmail.com".to_string(),            role : crate::models::UserRole::Admin,
            max_users: None,
            system: vec![ crate::models::System::Invoicing ],
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

    #[async_test]
    async fn create_session_test(){
        let mongo = get_mongo().await;
        let user = get_user(&mongo, &String::from("vasilis")).await.unwrap();
        let session_str = create_session(&user, &mongo).await.unwrap();
        println!("{:?}", session_str);
    }

    #[async_test]
    async fn update_session_dt_test(){
        let mongo = get_mongo().await;
        let user = get_user(&mongo, &String::from("vasilis")).await.unwrap(); 
        let res = update_session_dt(user.id.unwrap().to_string(), &mongo).await;
        assert!(res.is_ok());
    }
}



