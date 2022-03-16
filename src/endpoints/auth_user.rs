use rocket::State;
use rocket::serde::json::Json;
use crate::services::user_service;
use crate::models::{api_response::ApiError, user::LoginUser, sessions::SessionId}; 

#[get("/logout")]
pub async fn logout(mongo: &State<mongodb::Database>,session_id : SessionId) -> Result<(), Json<ApiError>>{
    user_service::logout_service(mongo, session_id.session_id).await.map_err( |e| Json( ApiError{ error:e}))
}


#[post("/login",format = "json", data="<user_login_js>")]
pub async fn login(user_login_js: Json<LoginUser>, mongo: &State<mongodb::Database>) -> Result<Json<SessionId>, Json<ApiError>>{
    let login_user = user_login_js.into_inner();
    let session_id = user_service::login_service(mongo, login_user).await.map_err( |e| Json( ApiError{ error:e}))?;
    Ok(Json(SessionId{session_id}))
}