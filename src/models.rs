use chrono::{DateTime, Local};
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
    pub system: Vec<System>,
    pub database: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum System {
    Invoicing
}

pub struct UserSession {
    pub id: i64,
    pub user_id: String,
    pub database: String,
    pub system: Vec<System>,
    pub dt:  DateTime<Local>,
}

#[derive(Debug)]
pub struct DbClients {
    pub redis : redis::Client,
    pub mongo : mongodb::Database,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError{
    pub error: String 
}
