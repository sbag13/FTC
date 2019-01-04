use crate::db_queries::insert_user;
use crate::db_queries::DbConn;
use crate::db_structs::InsertableUser;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::json::Json;
use validator::validate_email;

const REASON_USER_EXISTS: &'static str = "User already exists!";
const REASON_BAD_EMAIL: &'static str = "Invalid email!";

#[post("/registration", format = "json", data = "<user>")]
pub fn registration_post(
    user: Json<InsertableUser>,
    conn: DbConn,
) -> Result<Custom<Json<InsertableUser>>, Status> {
    if validate_email(&user.mail) == false {
        return Err(Status::new(400, REASON_BAD_EMAIL));
    }

    match insert_user(&conn, &user) {
        Ok(_row_count) => return Ok(Custom(Status::Created, user)),
        Err(_err) => return Err(Status::new(409, REASON_USER_EXISTS)),
    }
}

#[get("/registration")]
pub fn registration_get() -> Status {
    Status::MethodNotAllowed
}

#[put("/registration")]
pub fn registration_put() -> Status {
    Status::MethodNotAllowed
}

#[delete("/registration")]
pub fn registration_delete() -> Status {
    Status::MethodNotAllowed
}