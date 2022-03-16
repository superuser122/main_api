use rocket::State;
use rocket::serde::json::Json;
use crate::services::{session_service::{validate_session}, user_service};
use crate::models::{api_response::ApiError, user::{LoginUser, User, UserRole}, sessions::SessionId}; 

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

#[post("/create",format = "json", data="<user>")]
pub async fn create(user: Json<User>, mongo: &State<mongodb::Database>, session_id: SessionId) -> Result<(), Json<ApiError>>{
    let user  = user.into_inner();
    let session = validate_session(&session_id.session_id, mongo).await.map_err( |e| Json( ApiError{ error:e}))?;
    match user.role {
        UserRole::Admin => user_service::create_user(mongo, user).await.map_err( |e| Json( ApiError{ error:e}))?,
        UserRole::Owner =>{
            if user.role != UserRole::User && user.database != session.user.database {
                return Err(Json(ApiError{ error: String::from("Unauthorized user creation")}));
            }
            user_service::create_user(mongo, user).await.map_err( |e| Json( ApiError{ error:e}))?;
        },
        UserRole::User => return Err(Json(ApiError{ error: String::from("Unauthorized user creation")}))
    }
    Ok(())
}

#[get("/readself")]
pub async fn read_self(mongo: &State<mongodb::Database>, session_id : SessionId) -> Result<Json<User>, Json<ApiError>>{
    let session = validate_session(&session_id.session_id, mongo).await.map_err( |e| Json( ApiError{ error:e}))?;
    Ok(Json(session.user))
}
