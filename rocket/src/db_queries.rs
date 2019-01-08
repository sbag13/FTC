use crate::db_structs::*;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::result::QueryResult;
use diesel::{delete, insert_into, update, RunQueryDsl};
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

pub fn insert_offer(conn: &diesel::SqliteConnection, offer: InsertableOffer) -> QueryResult<i32> {
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

pub fn get_offer_by_id(conn: &diesel::SqliteConnection, got_id: i32) -> QueryResult<Offer> {
    use crate::schema::offers::dsl::*;
    offers.find(got_id).get_result(conn)
}

pub fn update_offer(conn: &diesel::SqliteConnection, offer: Offer) -> QueryResult<usize> {
    use crate::schema::offers::dsl::*;
    update(offers.find(offer.id)).set(&offer).execute(conn)
}

pub fn offer_delete(conn: &diesel::SqliteConnection, got_id: i32) -> QueryResult<usize> {
    use crate::schema::offers::dsl::*;
    delete(offers.find(got_id)).execute(conn)
}

pub fn insert_transaction(
    conn: &diesel::SqliteConnection,
    transaction: InsertableTransaction,
) -> QueryResult<usize> {
    use crate::schema::transactions::dsl::*;
    insert_into(transactions).values(transaction).execute(conn)
}

pub fn get_transaction_by_offer_id(
    conn: &diesel::SqliteConnection,
    got_id: i32,
) -> QueryResult<Transaction> {
    use crate::schema::transactions::dsl::*;
    transactions.filter(offer_id.eq(got_id)).get_result(conn)
}

pub fn update_transaction(
    conn: &diesel::SqliteConnection,
    transaction: Transaction,
) -> QueryResult<usize> {
    use crate::schema::transactions::dsl::*;
    update(transactions.find(transaction.id))
        .set(&transaction)
        .execute(conn)
}
