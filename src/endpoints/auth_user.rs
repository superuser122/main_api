use rocket::State;
use rocket::serde::json::Json;
use crate::services::user_service;
use crate::models::{api_response::{ApiResponse, ApiError}, user::LoginUser, sessions::SessionId}; 

#[get("/logout")]
pub async fn logout(mongo: &State<mongodb::Database>,session_id : SessionId) -> Json<ApiResponse<Vec<ApiError>>> {
    let resp = user_service::logout_service(mongo, session_id.session_id).await;
    match resp {
        Ok(_) => Json(ApiResponse::ok()),
        Err(error) => Json(ApiResponse::error(vec![error])),
    }

}


#[post("/login",format = "json", data="<user_login_js>")]
pub async fn login(user_login_js: Json<LoginUser>, mongo: &State<mongodb::Database>) -> Json<ApiResponse<SessionId>> {
    let login_user = user_login_js.into_inner();
    let session_id = user_service::login_service(mongo, login_user).await;
    match session_id {
        Ok(id) => {
            let resp = ApiResponse::ok_with_body(SessionId{ session_id: id});
            Json(resp)
        },
        Err(error) => {
            let resp = ApiResponse::error(vec![error]);
            Json(resp)
        },
    }
}

