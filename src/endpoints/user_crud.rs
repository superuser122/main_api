use rocket::State;
use rocket::serde::json::Json;
use crate::{services::{session_service::{validate_session}, user_service}, };
use crate::models::{api_response::{ApiResponse, ApiError}, user::{User, UserRole}, sessions::SessionId}; 
use bcrypt::hash;

#[post("/user/create",format = "json", data="<user>")]
pub async fn create(user: Json<User>, mongo: &State<mongodb::Database>, session_id: SessionId) -> Json<ApiResponse<Vec<ApiError>>>{
    let user  = user.into_inner();
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    match session.user.role {
        UserRole::Admin => {
            match user_service::create_user(mongo, user).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::Owner =>{
            if user.role != UserRole::User || user.database != session.user.database {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized operation"))]));
            }
            let user_num = match user_service::get_users_num(mongo, session.user.database ).await {
                Ok(num) => num,
                Err(error) => return Json(ApiResponse::error(vec![error])),
            };
            let max_users = match session.user.max_users {
                Some(m) => m,
                None => return Json(ApiResponse::error(vec![(String::from("11"), String::from("No max users value"))])),
            };
            if max_users >= user_num {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Max number of users"))]));
            }
            if session.user.expiration_dt != user.expiration_dt {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Expiration date conflict"))]));
            }
            match user_service::create_user(mongo, user).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::User => return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized operation"))]))
    }
}

#[get("/user/delete/<user_name>")]
pub async fn delete(mongo: &State<mongodb::Database>, user_name: String, session_id: SessionId) -> Json<ApiResponse<Vec<ApiError>>>{
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    let user = match user_service::get_user(mongo, &user_name).await{
        Ok(user) => user,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };

    let user_id = user.id.unwrap().to_string();
    match session.user.role {
        UserRole::Admin => {
            match user_service::delete_user(mongo, user_id).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::Owner =>{
            if user.role != UserRole::User && user.database != session.user.database {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized operation"))]));
            }
            match user_service::delete_user(mongo, user_id).await {
                Ok(_) => return Json(ApiResponse::ok()),
                Err(error) => return Json(ApiResponse::error(vec![error])),
            }
        },
        UserRole::User => return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized operation"))]))
    }
}


#[get("/user/read/<user_name>")]
pub async fn read_user(mongo: &State<mongodb::Database>, session_id : SessionId, user_name: String) -> Json<ApiResponse<User>>{
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    let user = match user_service::get_user(mongo, &user_name).await{
        Ok(user) => user,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    match session.user.role {
        UserRole::Admin => Json(ApiResponse::ok_with_body(user)),
        UserRole::Owner =>{
            if user.database != session.user.database {
                return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized user request"))]));
            }
            return Json(ApiResponse::ok_with_body(user));
        },
        UserRole::User => return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized user request"))]))
    }
}

#[get("/user/readself")]
pub async fn read_self(mongo: &State<mongodb::Database>, session_id: SessionId) -> Json<ApiResponse<User>>{
    match validate_session(&session_id.session_id, mongo).await{
        Ok(mut session) => {
            session.user.id = None;
            session.user.password = String::new();
            return Json(ApiResponse::ok_with_body(session.user))
        },
        Err(error) => return Json(ApiResponse::error(vec![error])),
    }
}

#[get("/user/expdate/<database>/<date>")]
pub async fn update_expdate(mongo: &State<mongodb::Database>, session_id: SessionId, database: String, date: String ) -> Json<ApiResponse<Vec<ApiError>>>{
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    if session.user.role != UserRole::Admin {
        return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized oparation"))]));
    }
    match user_service::update_expitation(mongo, database, date).await {
        Ok(_) => return Json(ApiResponse::ok()),
        Err(error) => return Json(ApiResponse::error(vec![error])),
    }
}

#[post("/user/updateself",format = "json", data="<user>")]
pub async fn update_self(user: Json<User>, session_id: SessionId, mongo: &State<mongodb::Database>) -> Json<ApiResponse<Vec<ApiError>>>{
    let updated_user = user.into_inner();
    let mut session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    session.user.password = match hash(updated_user.password, 4) {
        Ok(p) => p,
        Err(error) => return Json(ApiResponse::error(vec![(String::from("11"), error.to_string())]))
    };
    session.user.email = updated_user.email;
    if session.user.role == UserRole::Admin{
        session.user.database = updated_user.database;
        session.user.system = updated_user.system.to_owned();
    }
    match user_service::update_user(mongo, session.user).await{
        Ok(_) => return Json(ApiResponse::ok()),
        Err(error) => return Json(ApiResponse::error(vec![error])),
    }
}

#[post("/user/update",format = "json", data="<user>")]
pub async fn update(user: Json<User>, session_id: SessionId, mongo: &State<mongodb::Database>) -> Json<ApiResponse<Vec<ApiError>>>{
    let session = match validate_session(&session_id.session_id, mongo).await {
        Ok(session) => session,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    let updated_user = user.into_inner();
    let mut old_user = match user_service::get_user(mongo, &updated_user.user_name).await {
        Ok(user) => user,
        Err(error) => return Json(ApiResponse::error(vec![error])),
    };
    old_user.email = updated_user.email;
    old_user.system = updated_user.system.to_owned();

    match session.user.role {
        UserRole::Admin => {
            old_user.password = match hash(updated_user.password, 4) {
                Ok(p) => p,
                Err(error) => return Json(ApiResponse::error(vec![(String::from("11"), error.to_string())]))
            };
            old_user.database = updated_user.database;
            old_user.max_users = updated_user.max_users;
            old_user.role = updated_user.role;
            old_user.expiration_dt = updated_user.expiration_dt;
        },
        UserRole::User => return Json(ApiResponse::error(vec![(String::from("11"), String::from("Unauthorized user request"))])),
        _ => {}
    }

    match user_service::update_user(mongo, old_user).await{
        Ok(_) => return Json(ApiResponse::ok()),
        Err(error) => return Json(ApiResponse::error(vec![error])),
    }
}


