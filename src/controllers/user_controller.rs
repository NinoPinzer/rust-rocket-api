use rocket::{delete, get, http::Status, post, serde::json::Json};

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

#[get("/users/<id>")]
pub async fn get_user_by_id(id: i32, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::get_user_by_id(id, db).await {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}

#[delete("/users/<id>")]
pub async fn delete_user_by_id(id: i32, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::delete_user_by_id(id, db).await {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}

#[post("/users/<id>", data = "<new_user>")]
pub async fn update_user_by_id(id: i32, new_user: Json<InsertableUser>, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::update_user_by_id(id, new_user.into_inner(), db).await {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}