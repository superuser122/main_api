use rocket::State;
use rocket::serde::json::Json;
use crate::{services::{session_service::{validate_session}, user_service}, };
use crate::models::{api_response::{ApiResponse, ApiError}, user::{User, UserRole}, sessions::SessionId}; 


#[post("/create",format = "json", data="<user>")]
pub async fn create(user: Json<User>, mongo: &State<mongodb::Database>, session_id: SessionId) -> Json<ApiResponse<Vec<ApiError>>>{
    let user  = user.into_inner();
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    match user.role {
        UserRole::Admin => {
            match user_service::create_user(mongo, user).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::Owner =>{
            if user.role != UserRole::User && user.database != session.user.database {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized user creation"))]));
            }
            match user_service::create_user(mongo, user).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::User => return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized user creation"))]))
    }
}

#[get("/readself")]
pub async fn read_self(mongo: &State<mongodb::Database>, session_id : SessionId) -> Json<ApiResponse<User>>{
    match validate_session(&session_id.session_id, mongo).await{
        Ok(session) => return Json(ApiResponse::ok_with_body(session.user)),
        Err(error) => return Json(ApiResponse::error(vec![error])),
    }
}
