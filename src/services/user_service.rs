use crate::repositories::user_repositories;
use rust_rocket_api::models::user::{User, InsertableUser};
use crate::config::database::UsersDbConn;

pub async fn get_all_users(db: UsersDbConn) -> Vec<User> {
    user_repositories::fetch_users(db).await
}


pub async fn create_user(user: InsertableUser, db: UsersDbConn) -> User {
    user_repositories::insert_user(user, db).await
}