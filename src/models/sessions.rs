use chrono::{DateTime, Utc};
use rocket::http::Status;
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use crate::models::{user::User,  api_response::ApiError};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user: User,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub dt:  DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionId{
    pub session_id: String
}
#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionId{
    type Error = Json<ApiError>;

    async fn from_request(req: &'r Request<'_>) ->Outcome<Self, Self::Error>{
        match req.headers().get_one("session"){
            Some(sess_id) => Outcome::Success(SessionId{session_id: sess_id.to_string()}),
            None => Outcome::Failure((Status::Unauthorized, Json(ApiError{ error: String::from("Unauthorized")}))),
        }
    }
}