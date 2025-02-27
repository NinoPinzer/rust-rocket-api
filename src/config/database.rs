use rocket_sync_db_pools::database;
use diesel::PgConnection;

#[database("users_db")]
pub struct UsersDbConn(PgConnection);