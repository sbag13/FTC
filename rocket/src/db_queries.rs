use crate::db_structs::*;
use diesel::{insert_into, RunQueryDsl};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::QueryResult;
use rocket_contrib::databases;

#[database("sqlite_db")]
pub struct DbConn(databases::diesel::SqliteConnection);

pub fn insert_user(conn: &diesel::SqliteConnection, user: &InsertableUser) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;
    insert_into(users).values(user).execute(conn)
}

pub fn select_user_by_mail(
    conn: &diesel::SqliteConnection,
    user_mail: &String,
) -> QueryResult<(InsertableUser)> {
    use crate::schema::users::dsl::*;
    users.find(user_mail).get_result::<InsertableUser>(conn)
}

pub fn insert_auction(
    conn: &diesel::SqliteConnection,
    auction: InsertableAuction,
) -> QueryResult<i32> {
    use crate::schema::auctions::dsl::*;
    match insert_into(auctions).values(auction).execute(conn) {
        Ok(_) => auctions.select(id).order(id.desc()).first(conn),
        Err(e) => return Err(e),
    }
}

pub fn insert_buynow(
    conn: &diesel::SqliteConnection,
    buynow: InsertableBuynow,
) -> QueryResult<i32> {
    use crate::schema::buynows::dsl::*;
    match insert_into(buynows).values(buynow).execute(conn) {
        Ok(_) => buynows.select(id).order(id.desc()).first(conn),
        Err(e) => return Err(e),
    }
}
