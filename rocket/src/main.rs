#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod schema;
mod db_queries;
mod session;
mod db_structs;

fn main() {
    println!("test");
    rocket::ignite()
        .attach(db_queries::DbConn::fairing())
        .mount("/", routes![session::registration])
        .launch();
}
