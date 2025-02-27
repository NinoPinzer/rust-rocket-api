use diesel::prelude::*;
use rust_rocket_api::schema::users::dsl::*;
use crate::config::database::UsersDbConn;
use rust_rocket_api::models::user::{User, InsertableUser};

pub async fn fetch_users(db: UsersDbConn) -> Vec<User> {
    db.run(|conn| {
        users.load::<User>(conn)
    }).await.unwrap()
}

pub async fn insert_user(new_user: InsertableUser, db: UsersDbConn) -> User {
    db.run(move |conn| {
        diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)
    }).await.unwrap()
}