#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate jsonwebtoken;
extern crate validator;
#[macro_use]
extern crate json;

mod db_queries;
mod db_structs;
mod endpoints;
mod schema;

fn main() {
    println!("test");
    rocket::ignite()
        .attach(db_queries::DbConn::fairing())
        .mount(
            "/",
            routes![
                endpoints::registration_post,
                endpoints::registration_get,
                endpoints::registration_delete,
                endpoints::registration_put,
                endpoints::login_post,
                endpoints::login_get,
                endpoints::login_put,
                endpoints::login_delete,
                endpoints::offer_post,
                endpoints::all_offers_get,
                endpoints::my_offers_get,
                endpoints::offer_patch,
                endpoints::offer_delete,
                endpoints::offer_get,
                endpoints::offer_buy
            ],
        )
        .launch();
}
