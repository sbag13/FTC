use diesel::insert_into;
use diesel::result::QueryResult;
use diesel::RunQueryDsl;
use rocket_contrib::databases;
use crate::db_structs::InsertableUser;
use crate::schema::users::dsl::*;

#[database("sqlite_db")]
pub struct DbConn(databases::diesel::SqliteConnection);

pub fn insert_user(conn: &diesel::SqliteConnection, user: &InsertableUser) -> QueryResult<usize> {
    insert_into(users).values(user).execute(conn)
}