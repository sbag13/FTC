use crate::schema::*;

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub mail: String,
    pub password: String,
}

#[derive(Serialize, Insertable, Deserialize, Queryable)]
#[table_name = "offers"]
pub struct InsertableOffer {
    pub type_: String,
    pub description: String,
    pub price: f32,
    pub date_amount: i32,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Debug)]
pub struct Offer {
    pub id: i32,
    pub type_: String,
    pub description: String,
    pub price: f32,
    pub date_amount: i32,
}

impl Offer {
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn print(&self) {
        println!("{:?}", self);
    }
    pub fn as_json(&self) -> String {
        json!({
            "type": self.type_,
            "description": self.description,
            "price": self.price,
            "date": self.date_amount
        }).to_string()
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    pub fn contains_description(&self, desc: &String) -> bool {
        self.get_description().contains(desc.as_str())
    }
    pub fn filter_by_price_min(&self, min_price: f32) -> bool {
        self.get_price() > min_price
    }
    pub fn filter_by_price_max(&self, max_price: f32) -> bool {
        self.get_price() < max_price
    }
    pub fn filter_by_type(&self, got_type: &String) -> bool {
        if got_type.as_str() == self.type_.as_str() { true }
        else { false }
    }
}

#[derive(Serialize)]
pub struct OfferId {
    pub offer_id: i32,
}
