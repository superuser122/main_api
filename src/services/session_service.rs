use crate::models::{user::User, sessions::UserSession};
use bson::Document;
use mongodb::{bson::{doc, oid::ObjectId,}, Collection};
use chrono::{Utc, Duration};
use std::str::FromStr;

pub async fn create_session(user: &User, database : &mongodb::Database,) -> Result<String, (String,String)>{
    if user.expiration_dt < Utc::now(){
        return Err((String::from("11"),"Subscription has expired".to_string()));
    }
    let session_collection = database.collection("sessions");
    let session = UserSession{
        id: None,
        user: user.clone(),
        dt: Utc::now()
    };
    let session_ser = bson::to_bson(&session).map_err(|e| (String::from("11"),e.to_string()))?;
    let session_doc = match session_ser.as_document(){
        Some(doc) => doc,
        None => return Err((String::from("11"),"Unable to parse doc!".to_string()))
    };
    let user_id = match user.id {
        Some(id) => id.to_string(),
        None => return Err((String::from("11"),String::from("User id is not set")))
    };
    let _ = delete_sessions(user_id, &session_collection).await?;
    let ins_id =session_collection.insert_one(session_doc.to_owned(),  None).await.map_err(|e| (String::from("11"),e.to_string()))?;
    Ok(ins_id.inserted_id.as_object_id().unwrap().to_string())

}

pub async fn delete_sessions(user_id: String, session_collection: &Collection<Document>)-> Result<(), (String,String)>{
    let filter = doc!{"user": { "_id" : user_id}};
    let _ = session_collection.delete_many(filter, None).await.map_err(|e| (String::from("11"), e.to_string()))?;
    Ok(())
    
}

pub async fn delete_session(session_id: &String, session_collection: &Collection<Document>)-> Result<(), (String, String)>{
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|_e| (String::from("11"), "Session id incorrect".to_string()))?;
    let filter = doc!{"_id": obj_id};
    let _ = session_collection.delete_one(filter, None).await.map_err(|e| (String::from("11"),e.to_string()))?;
    Ok(())

}

pub async fn get_session(session_id: &String, database : &mongodb::Database)-> Result<Option<UserSession>, (String, String)>{
    let session_collection = database.collection::<UserSession>("sessions");
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|_e| (String::from("11"), "Session id incorrect".to_string()))?;
    let session = session_collection.find_one(doc! {"_id" : obj_id },None).await.map_err(|e| (String::from("11"), e.to_string()))?;
    Ok(session)
}

pub async fn update_session_dt(session_id: String, database : &mongodb::Database)-> Result<(), (String, String)>{
    let session_collection = database.collection::<UserSession>("sessions");
    let obj_id = ObjectId::from_str(session_id.as_str()).map_err(|_e| (String::from("11"), "Session id incorrect".to_string()))?;
    let filter =doc!{"_id" : obj_id };
    let update = doc!{ "$currentDate" : {"dt" : true}};
    session_collection.update_one(filter, update, None).await.map_err(|e| (String::from("11"), e.to_string()))?;
    Ok(())
}

pub async fn validate_session(session_id: &String, database : &mongodb::Database) -> Result<UserSession, (String, String)> {
    let session_opt = get_session(&session_id, database).await?;
    let session = match session_opt {
        Some(s) => s,
        None => return Err((String::from("11"),"No session found".to_string()))
    };
    let valid_till = session.dt + Duration::hours(1);
    if valid_till < Utc::now() {
        return Err((String::from("11"),"Session has expired".to_string()));
    }
    if session.user.expiration_dt < Utc::now() {
        return Err((String::from("11"),"Subscription has expired".to_string()));
    }
    update_session_dt(session_id.clone(), database).await?;
    Ok(session)
}


//Unit testing

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use crate::services::user_service::get_user;

    async fn get_mongo() -> mongodb::Database { 
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").unwrap();
        mongodb::Client::with_uri_str(mongo_url)
                            .await.unwrap()
                            .database("userdb")
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