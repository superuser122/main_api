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

impl User{
    pub fn clone(&self) -> Self{
        Self{
            id : self.id.to_owned(),
            user_name : self.user_name.clone(),
            password : self.password.clone(),
            email: self.email.clone(),
            role: self.role,
            max_users: self.max_users,
            system: self.system.to_owned(),
            database: self.database.clone(),
            expiration_dt: self.expiration_dt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser{
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Owner,
    User,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum System {
    Invoicing
}