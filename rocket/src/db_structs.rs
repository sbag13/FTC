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

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Debug)]
pub struct Auction {
    pub id: i32,
    pub description: String,
    pub price: f32,
    pub date: i32,
}

impl DbOffer for Auction {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn print(&self) {
        println!("{:?}", self);
    }
    fn as_json(&self) -> String {
        json!({
            "type": "auction",
            "description": self.description,
            "price": self.price,
            "date": self.date
        }).to_string()
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn is_type(&self, got_type: &str) -> bool {
        if got_type == "auction" { true }
        else { false }
    }
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Debug)]
pub struct Buynow {
    pub id: i32,
    pub description: String,
    pub price: f32,
    pub amount: i32,
}

impl DbOffer for Buynow {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn print(&self) {
        println!("{:?}", self);
    }
    fn as_json(&self) -> String {
        json!({
            "type": "buynow",
            "description": self.description,
            "price": self.price,
            "amount": self.amount
        }).to_string()
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn is_type(&self, got_type: &str) -> bool {
        if got_type == "buynow" { true }
        else { false }
    }
}

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "buynows"]
pub struct InsertableBuynow {
    pub description: String,
    pub price: f32,
    pub amount: i32,
}

pub trait DbOffer {
    fn contains_description(&self, desc: &String) -> bool {
        self.get_description().contains(desc.as_str())
    }
    fn filter_by_price_min(&self, min_price: f32) -> bool {
        self.get_price() > min_price
    }
    fn filter_by_price_max(&self, max_price: f32) -> bool {
        self.get_price() < max_price
    }
    fn filter_by_type(&self, got_type: &String) -> bool {
        self.is_type(got_type.as_str())
    }
    fn is_type(&self, got_type: &str) -> bool;
    fn get_description(&self) -> String;
    fn print(&self);
    fn as_json(&self) -> String;
    fn get_price(&self) -> f32;
}

#[derive(Serialize)]
pub struct OfferId {
    pub offer_id: i32,
}
