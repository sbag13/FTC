#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate jsonwebtoken;
extern crate validator;

mod db_queries;
mod db_structs;
mod schema;
mod session;

fn main() {
    println!("test");
    rocket::ignite()
        .attach(db_queries::DbConn::fairing())
        .mount(
            "/",
            routes![
                session::registration_post,
                session::registration_get,
                session::registration_delete,
                session::registration_put,
                session::login_post,
                session::login_get,
                session::login_put,
                session::login_delete
            ],
        )
        .launch();
}
