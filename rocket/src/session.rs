use crate::db_queries::insert_user;
use crate::db_queries::DbConn;
use crate::db_queries::select_user_by_mail;
use crate::db_structs::InsertableUser;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::json::Json;
use validator::validate_email;
use jsonwebtoken;

const REASON_USER_EXISTS: &'static str = "User already exists!";
const REASON_BAD_EMAIL: &'static str = "Invalid email!";

//
//
// Login
//

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    mail: String
}

#[post("/login", format = "json", data = "<given_user>")]
pub fn login_post(
    given_user: Json<InsertableUser>,
    conn: DbConn,
) -> Status {
    if validate_email(&given_user.mail) == false {
        return Status::new(400, REASON_BAD_EMAIL);
    }

    match select_user_by_mail(&conn, &given_user.mail) {
        Ok(db_user) => {
            if db_user.password == given_user.password {
                return Status::Ok;
            }
            else {
                return Status::Unauthorized;
            }
        }
        Err(_err) => return Status::NotFound,  //TODO change
    }
}

#[get("/login")]
pub fn login_get() -> Status {
    Status::MethodNotAllowed
}

#[put("/login")]
pub fn login_put() -> Status {
    Status::MethodNotAllowed
}

#[delete("/login")]
pub fn login_delete() -> Status {
    Status::MethodNotAllowed
}

//
//
// Registration
//

#[post("/registration", format = "json", data = "<user>")]
pub fn registration_post(
    user: Json<InsertableUser>,
    conn: DbConn,
) -> Status {
    if validate_email(&user.mail) == false {
        return Status::new(400, REASON_BAD_EMAIL);
    }

    match insert_user(&conn, &user) {
        Ok(_row_count) => return Status::Created,
        Err(_err) => return Status::new(409, REASON_USER_EXISTS),
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