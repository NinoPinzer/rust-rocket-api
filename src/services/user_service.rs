use crate::repositories::user_repositories;
use rust_rocket_api::models::user::{User, InsertableUser};
use crate::config::database::UsersDbConn;

pub async fn get_all_users(db: UsersDbConn) -> Vec<User> {
    user_repositories::fetch_users(db).await
}

pub async fn create_user(user: InsertableUser, db: UsersDbConn) -> User {
    user_repositories::insert_user(user, db).await
}

pub async fn get_user_by_id(id: i32, db: UsersDbConn) -> Option<User> {
    user_repositories::fetch_user_by_id(id, db).await
}

pub async fn update_user_by_id(user_id: i32, new_user: InsertableUser, db: UsersDbConn) -> Option<User> {
    user_repositories::update_user_by_id(user_id, new_user, db).await
}

pub async fn delete_user_by_id(id: i32, db: UsersDbConn) -> Option<User> {
    user_repositories::delete_user_by_id(id, db).await
}

