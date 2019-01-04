use diesel::insert_into;
use diesel::select;
use diesel::query_dsl::QueryDsl;
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

pub fn select_user_by_mail(conn: &diesel::SqliteConnection, user_mail: &String) -> QueryResult<(InsertableUser)> {
    users.find(user_mail).get_result::<InsertableUser>(conn)
}