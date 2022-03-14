use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;


#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_users : Option<u8>,
    pub system: Vec<System>,
    pub database: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub expiration_dt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser{
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRole {
    Admin,
    Owner,
    User,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum System {
    Invoicing
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub database: String,
    pub system: Vec<System>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_users : Option<u8>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub dt:  DateTime<Utc>,
}

#[derive(Debug)]
pub struct DbClients {
    pub redis : redis::Client,
    pub mongo : mongodb::Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionId{
    pub session_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError{
    pub error: String 
}
