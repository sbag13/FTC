use crate::db_queries;
use crate::db_queries::DbConn;
use crate::db_structs::*;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use rocket::http::{Cookie, Cookies, Status};
use rocket::response::status::Custom;
use rocket_contrib::json::Json;
use validator::validate_email;

const REASON_USER_EXISTS: &'static str = "User already exists!";
const REASON_BAD_EMAIL: &'static str = "Invalid email!";
const TOKEN_KEY: &'static str = "secret";

//
//
// Authorization
//

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    mail: String,
}

fn authorize(cookies: &Cookies) -> Result<String, ()> {
    let sess_token = match cookies.get("jwt") {
        Some(cookie) => cookie.value(),
        None => return Err(()),
    };
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let token_data = match decode::<Claims>(&sess_token, TOKEN_KEY.as_ref(), &validation) {
        Ok(token) => token,
        Err(_) => return Err(()),
    };

    Ok(token_data.claims.mail)
}

//
//
// Offers
//

#[derive(FromForm)]
pub struct Offer {
    #[form(field = "type")]
    api_type: String,
    description: String,
    price: f32,
    date: u64,
    amount: u32,
}

#[post("/offers", format = "json", data = "<offer>")]
pub fn offer_post(
    cookies: Cookies,
    offer: String,
    conn: DbConn,
) -> Result<Custom<Json<OfferId>>, Status> {
    // authorize
    let user_mail = match authorize(&cookies) {
        Err(()) => return Err(Status::Unauthorized),
        Ok(mail) => mail,
    };

    let offer_json: serde_json::Value = match serde_json::from_str(offer.as_ref()) {
        Ok(json) => json,
        _ => return Err(Status::BadRequest),
    };

    if offer_json["description"].is_null()
        || offer_json["price"].is_null()
        || offer_json["type"].is_null()
    {
        println!("NULL");
        return Err(Status::BadRequest);
    }

    match offer_json["type"].as_str() {
        None => return Err(Status::BadRequest),
        Some("auction") => return handle_auction(&conn, offer_json, user_mail),
        Some("buynow") => return handle_buynow(&conn, offer_json, user_mail),
        Some(_) => return Err(Status::BadRequest),
    }
}

fn handle_auction(
    conn: &diesel::SqliteConnection,
    offer_json: serde_json::Value,
    user_mail: String,
) -> Result<Custom<Json<OfferId>>, Status> {
    if offer_json["date"].is_null() {
        return Err(Status::BadRequest);
    }
    let auction = InsertableAuction {
        description: offer_json["description"].as_str().unwrap().to_string(),
        price: offer_json["price"].as_f64().unwrap() as f32,
        date: offer_json["date"].as_i64().unwrap() as i32,
    };

    let id = match db_queries::insert_auction(conn, auction) {
        Ok(db_id) => db_id,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Custom(Status::Created, Json(OfferId { offer_id: id })))
}

fn handle_buynow(
    conn: &diesel::SqliteConnection,
    offer_json: serde_json::Value,
    user_mail: String,
) -> Result<Custom<Json<OfferId>>, Status> {
    if offer_json["amount"].is_null() {
        return Err(Status::BadRequest);
    }

    let buynow = InsertableBuynow {
        description: offer_json["description"].as_str().unwrap().to_string(),
        price: offer_json["price"].as_f64().unwrap() as f32,
        amount: offer_json["amount"].as_i64().unwrap() as i32,
    };

    let id = match db_queries::insert_buynow(conn, buynow) {
        Ok(db_id) => db_id,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Custom(Status::Created, Json(OfferId { offer_id: id })))
}

//
//
// Login
//

fn generate_token(mail: &String) -> String {
    let header = Header::default();
    let claims = Claims { mail: mail.clone() };
    return encode(&header, &claims, TOKEN_KEY.as_ref()).unwrap();
}

#[post("/login", format = "json", data = "<given_user>")]
pub fn login_post(mut cookies: Cookies, given_user: Json<InsertableUser>, conn: DbConn) -> Status {
    if validate_email(&given_user.mail) == false {
        return Status::new(400, REASON_BAD_EMAIL);
    }

    match db_queries::select_user_by_mail(&conn, &given_user.mail) {
        Ok(db_user) => {
            if db_user.password == given_user.password {
                let token = generate_token(&given_user.mail);
                cookies.add(Cookie::new("jwt", token));
                return Status::Ok;
            } else {
                return Status::Unauthorized;
            }
        }
        Err(_err) => return Status::NotFound,
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
pub fn registration_post(user: Json<InsertableUser>, conn: DbConn) -> Status {
    if validate_email(&user.mail) == false {
        return Status::new(400, REASON_BAD_EMAIL);
    }

    match db_queries::insert_user(&conn, &user) {
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
