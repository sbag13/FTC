use crate::schema::*;

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "auctions"]
pub struct InsertableAuction {
    pub description: String,
    pub price: f32,
    pub date: i32,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable)]
pub struct Auction {
    pub id: i32,
    pub description: String,
    pub price: f32,
    pub date: i32,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable)]
pub struct Buynow {
    pub id: i32,
    pub description: String,
    pub price: f32,
    pub amount: i32,
}

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "buynows"]
pub struct InsertableBuynow {
    pub description: String,
    pub price: f32,
    pub amount: i32,
}

#[derive(Serialize)]
pub struct OfferId {
    pub offer_id: i32,
}
