use rocket::{delete, get, http::Status, post, serde::json::Json};

use crate::config::database::UsersDbConn;
use crate::services::user_service;
use rust_rocket_api::models::user::{User, InsertableUser};
use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind;

#[get("/users")]
pub async fn get_users(db: UsersDbConn) -> Result<Json<Vec<User>>, Status> {
    match user_service::get_all_users(db).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/users", data = "<new_user>")]
pub async fn create_user(new_user: Json<InsertableUser>, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::create_user(new_user.into_inner(), db).await {
        Ok(user) => Ok(Json(user)),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users/<id>")]
pub async fn get_user_by_id(id: i32, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::get_user_by_id(id, db).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/users/<id>")]
pub async fn delete_user_by_id(id: i32, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::delete_user_by_id(id, db).await {
        Ok(user) => Ok(Json(user)),
        Err(DieselError::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/users/<id>", data = "<new_user>")]
pub async fn update_user_by_id(id: i32, new_user: Json<InsertableUser>, db: UsersDbConn) -> Result<Json<User>, Status> {
    match user_service::update_user_by_id(id, new_user.into_inner(), db).await {
        Ok(user) => Ok(Json(user)),
        Err(DieselError::NotFound) => Err(Status::NotFound),
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}