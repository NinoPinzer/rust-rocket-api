use crate::config::database::UsersDbConn;
use diesel::prelude::*;
use rust_rocket_api::models::user::{InsertableUser, User};
use rust_rocket_api::schema::users::dsl::*;

pub async fn fetch_users(db: UsersDbConn) -> Vec<User> {
    db.run(|conn| users.load::<User>(conn)).await.unwrap()
}

pub async fn insert_user(new_user: InsertableUser, db: UsersDbConn) -> User {
    db.run(move |conn| diesel::insert_into(users).values(new_user).get_result(conn))
        .await
        .unwrap()
}

pub async fn fetch_user_by_id(user_id: i32, db: UsersDbConn) -> Option<User> {
    db.run(move |conn| users.find(user_id).first::<User>(conn).optional())
        .await
        .unwrap()
}

pub async fn update_user_by_id(user_id: i32, new_user: InsertableUser, db: UsersDbConn) -> Option<User> {
    db.run(move |conn| {
        let user = users.find(user_id).first::<User>(conn).optional();

        if let Ok(_user) = &user {
            diesel::update(users.find(user_id))
                .set(new_user)
                .execute(conn)
                .unwrap();
        }

        user
    })
    .await
    .unwrap()
}

pub async fn delete_user_by_id(user_id: i32, db: UsersDbConn) -> Option<User> {
    db.run(move |conn| {
        let user = users.find(user_id).first::<User>(conn).optional();

        if let Ok(_user) = &user {
            diesel::delete(users.find(user_id)).execute(conn).unwrap();
        }

        user
    })
    .await
    .unwrap()
}
