use crate::config::database::UsersDbConn;
use diesel::prelude::*;
use diesel::result::Error;
use rust_rocket_api::models::user::{InsertableUser, User};
use rust_rocket_api::schema::users::dsl::*;

pub async fn fetch_users(db: UsersDbConn) -> Result<Vec<User>, Error> {
    db.run(|conn| users.load::<User>(conn)).await
}

pub async fn insert_user(new_user: InsertableUser, db: UsersDbConn) -> Result<User, Error> {
    db.run(move |conn| diesel::insert_into(users).values(new_user).get_result(conn)).await
}

pub async fn fetch_user_by_id(user_id: i32, db: UsersDbConn) -> Result<Option<User>, Error> {
    db.run(move |conn| users.find(user_id).first::<User>(conn).optional()).await
}

pub async fn update_user_by_id(
    user_id: i32,
    new_user: InsertableUser,
    db: UsersDbConn,
) -> Result<User, Error> {
    db.run(move |conn| {
        diesel::update(users.find(user_id))
            .set(new_user)
            .get_result::<User>(conn)
    })
    .await
}

pub async fn delete_user_by_id(user_id: i32, db: UsersDbConn) -> Result<User, Error> {
    db.run(move |conn| {
        diesel::delete(users.find(user_id)).get_result::<User>(conn)
    })
    .await
}
