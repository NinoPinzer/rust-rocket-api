use rocket::{get, post, http::Status, serde::json::Json};

use crate::config::database::UsersDbConn;
use crate::services::user_service;
use rust_rocket_api::models::user::{User, InsertableUser};

#[get("/users")]
pub async fn get_users(db: UsersDbConn) -> Json<Vec<User>> {
    Json(user_service::get_all_users(db).await)
}

#[post("/users", data = "<new_user>")]
pub async fn create_user(new_user: Json<InsertableUser>, db: UsersDbConn) -> Result<Json<User>, Status> {
    Ok(Json(user_service::create_user(new_user.into_inner(), db).await))
}
