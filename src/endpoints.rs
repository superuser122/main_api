use crate::models::*;
use rocket::State;
use rocket::serde::json::Json;

#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}


#[post("/login",format = "json", data="<user_login_js>")]
pub fn login(user_login_js: Json<LoginUser>, state: &State<DbClients>) -> Result<String, Json<ApiError>>{
    let login_user = user_login_js.into_inner();
    todo!()
}