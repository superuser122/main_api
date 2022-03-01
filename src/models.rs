use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;


#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub _id : ObjectId,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
    pub database: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser{
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
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

pub struct UserSession {
    pub id: i64,
    pub user_id: String,
    pub database: String,
    pub dt:  DateTime<Local>,
}
