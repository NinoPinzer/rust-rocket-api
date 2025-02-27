use crate::models::user::{User, NewUser};
use crate::repositories::user_repositories;

pub fn get_all_users() -> Vec<User> {
    return user_repositories::fetch_users();
}


pub fn create_user(new_user: NewUser) -> User {
    user_repositories::insert_user(new_user)
}