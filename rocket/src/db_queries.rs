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

pub fn insert_offer(
    conn: &diesel::SqliteConnection,
    offer: InsertableOffer,
) -> QueryResult<i32> {
    use crate::schema::offers::dsl::*;
    match insert_into(offers).values(offer).execute(conn) {
        Ok(_) => offers.select(id).order(id.desc()).first(conn),
        Err(e) => return Err(e),
    }
}

pub fn get_all_offers(conn: &diesel::SqliteConnection) -> QueryResult<Vec<Offer>> {
    use crate::schema::offers::dsl::*;
    offers.load(conn)
}

pub fn insert_owner(conn: &diesel::SqliteConnection, user_mail: &String, offer_id: i32) -> QueryResult<usize> {
    use crate::schema::owners::dsl::*;
    insert_into(owners).values((mail.eq(user_mail), id.eq(offer_id))).execute(conn)
}
