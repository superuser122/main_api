use crate::models::*;
use rocket::State;
use rocket::serde::json::Json;
use crate::services::*;



#[get("/logout")]
pub async fn logout(state: &State<DbClients>,session_id : SessionId) -> Result<(), Json<ApiError>>{
    logout_service(&state.mongo, session_id.session_id).await.map_err( |e| Json( ApiError{ error:e}))
}


#[post("/login",format = "json", data="<user_login_js>")]
pub async fn login(user_login_js: Json<LoginUser>, state: &State<DbClients>) -> Result<Json<SessionId>, Json<ApiError>>{
    let login_user = user_login_js.into_inner();
    let session_id = login_service(&state.mongo, login_user).await.map_err( |e| Json( ApiError{ error:e}))?;
    Ok(Json(SessionId{session_id}))
}

#[post("/create",format = "json", data="<user>")]
pub async fn create(user: Json<User>, state: &State<DbClients>, session_id: SessionId) -> Result<(), Json<ApiError>>{
    let user  = user.into_inner();
    let session = validate_session(&session_id.session_id, &state.mongo).await.map_err( |e| Json( ApiError{ error:e}))?;
    match user.role {
        UserRole::Admin => create_user(&state.mongo, user).await.map_err( |e| Json( ApiError{ error:e}))?,
        UserRole::Owner =>{
            if user.role != UserRole::User && user.database != session.user.database {
                return Err(Json(ApiError{ error: String::from("Unauthorized user creation")}));
            }
            create_user(&state.mongo, user).await.map_err( |e| Json( ApiError{ error:e}))?;
        },
        UserRole::User => return Err(Json(ApiError{ error: String::from("Unauthorized user creation")}))
    }
    Ok(())
}

#[get("/readself")]
pub async fn read_self(state: &State<DbClients>,session_id : SessionId) -> Result<Json<User>, Json<ApiError>>{
    let session = validate_session(&session_id.session_id, &state.mongo).await.map_err( |e| Json( ApiError{ error:e}))?;
    Ok(Json(session.user))
}
