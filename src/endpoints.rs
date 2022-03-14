use crate::models::*;
use rocket::State;
use rocket::serde::json::Json;
use crate::services::*;



#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}


#[post("/login",format = "json", data="<user_login_js>")]
pub async fn login(user_login_js: Json<LoginUser>, state: &State<DbClients>) -> Result<Json<SessionId>, Json<ApiError>>{
    let login_user = user_login_js.into_inner();
    let session_id = login_service(state, login_user).await.map_err( |e| Json( ApiError{ error:e}))?;
    Ok(Json(SessionId{session_id}))
}