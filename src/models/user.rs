use serde::{Serialize, Deserialize};
use diesel::{prelude::AsChangeset, Insertable, Queryable};
use crate::schema::users;


#[derive(Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    name: String,
    email: String,
}
