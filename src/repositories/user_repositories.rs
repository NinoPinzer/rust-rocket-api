use crate::models::user::{User, NewUser};

pub fn fetch_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "test@gmail.com".to_string()
        },
    
    ]
}

pub fn insert_user(new_user: NewUser) -> User {
    User {
        id: 3,
        name: new_user.name,
        email: new_user.email
    }
}