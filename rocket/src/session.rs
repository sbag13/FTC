use rocket_contrib::json::Json;
use crate::db_queries::DbConn;
use crate::db_queries::insert_user;
use crate::db_structs::InsertableUser;

#[get("/registration")]
pub fn registration(conn: DbConn) -> Json<InsertableUser> {
    //TODO get data from json
    let user = InsertableUser{
        mail: String::from("mmmm@mmm.pl"),
        password: String::from("password")
    };

    //TODO validate json

    match insert_user(&conn, &user) {
        Ok(_row_count) => return Json(user),
        _ => return Json(user), //TODO faile code
    }    
}