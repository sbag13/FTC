use crate::db_queries;
use crate::db_queries::DbConn;
use crate::db_structs::*;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::LenientForm;
use rocket::response::status::Custom;
use rocket_contrib::json::Json;
use std::time::{SystemTime, UNIX_EPOCH};
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

fn authorize(cookies: &Cookies, conn: &DbConn) -> Result<String, ()> {
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

    let mail = token_data.claims.mail;
    match db_queries::select_user_by_mail(conn, &mail) {
        Ok(_) => return Ok(mail),
        _ => return Err(()),
    }
}

//
//
// Offers
//

#[post("/offers", format = "json", data = "<offer>")]
pub fn offer_post(
    cookies: Cookies,
    offer: String,
    conn: DbConn,
) -> Result<Custom<Json<OfferId>>, Status> {
    // authorize
    let user_mail = match authorize(&cookies, &conn) {
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
    let offer = InsertableOffer {
        owner: user_mail,
        description: offer_json["description"].as_str().unwrap().to_string(),
        price: offer_json["price"].as_f64().unwrap() as f32,
        date_amount: offer_json["date"].as_i64().unwrap() as i32,
        type_: offer_json["type"].as_str().unwrap().to_string(),
    };

    let id = match db_queries::insert_offer(conn, offer) {
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

    let offer = InsertableOffer {
        owner: user_mail,
        description: offer_json["description"].as_str().unwrap().to_string(),
        price: offer_json["price"].as_f64().unwrap() as f32,
        date_amount: offer_json["amount"].as_i64().unwrap() as i32,
        type_: offer_json["type"].as_str().unwrap().to_string(),
    };

    let id = match db_queries::insert_offer(conn, offer) {
        Ok(db_id) => db_id,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Custom(Status::Created, Json(OfferId { offer_id: id })))
}

#[derive(FromForm, Debug, Clone)]
pub struct TypeExt {
    #[form(field = "type")]
    api_type: String,
}

fn all_offers(
    conn: DbConn,
    contains: Option<String>,
    price_min: Option<f32>,
    price_max: Option<f32>,
    ext_type: Option<LenientForm<TypeExt>>,
    cookies: Option<Cookies>,
) -> Result<Custom<String>, Status> {
    let mut user_mail: Option<String> = None;
    if cookies.is_some() {
        user_mail = match authorize(&cookies.unwrap(), &conn) {
            Err(()) => return Err(Status::Unauthorized),
            Ok(mail) => Some(mail),
        };
    }
    let got_type: Option<String> = match ext_type {
        Some(t) => Some(t.api_type.clone()),
        None => None,
    };
    if validate_filter_params(&got_type).is_err() {
        return Err(Status::BadRequest);
    };
    let offers_jsons_strings =
        get_filtered_offers(conn, contains, price_min, price_max, got_type, user_mail);
    Ok(Custom(
        Status::Ok,
        offers_jsons_strings
            .iter()
            .fold(String::new(), |folded, next| folded + next + "\n"),
    ))
}

#[get("/offers?<contains>&<price_min>&<price_max>&created_by_me&<ext_type..>")]
pub fn my_offers_get(
    cookies: Cookies,
    conn: DbConn,
    contains: Option<String>,
    price_min: Option<f32>,
    price_max: Option<f32>,
    ext_type: Option<LenientForm<TypeExt>>,
) -> Result<Custom<String>, Status> {
    all_offers(
        conn,
        contains,
        price_min,
        price_max,
        ext_type,
        Some(cookies),
    )
}

#[get("/offers?<contains>&<price_min>&<price_max>&<ext_type..>")]
pub fn all_offers_get(
    conn: DbConn,
    contains: Option<String>,
    price_min: Option<f32>,
    price_max: Option<f32>,
    ext_type: Option<LenientForm<TypeExt>>,
) -> Result<Custom<String>, Status> {
    all_offers(conn, contains, price_min, price_max, ext_type, None)
}

fn validate_filter_params(ext_type: &Option<String>) -> Result<(), ()> {
    if ext_type.is_some() {
        let got_type = ext_type.clone().unwrap();
        if got_type.as_str() != "auction" && got_type.as_str() != "buynow" {
            return Err(());
        }
    }
    Ok(())
}

fn get_filtered_offers(
    conn: DbConn,
    contains_opt: Option<String>,
    price_min: Option<f32>,
    price_max: Option<f32>,
    got_type: Option<String>,
    user: Option<String>,
) -> Vec<String> {
    let mut filters: Vec<Box<Fn(&Offer) -> bool>> = Vec::new();

    if contains_opt.is_some() {
        filters.push(Box::new(|offer: &Offer| -> bool {
            offer.contains_description(&contains_opt.clone().unwrap())
        }));
    }
    if price_min.is_some() {
        filters.push(Box::new(|offer: &Offer| -> bool {
            offer.filter_by_price_min(price_min.unwrap())
        }));
    }
    if price_max.is_some() {
        filters.push(Box::new(|offer: &Offer| -> bool {
            offer.filter_by_price_max(price_max.unwrap())
        }));
    }
    if got_type.is_some() {
        filters.push(Box::new(|offer: &Offer| -> bool {
            offer.filter_by_type(&got_type.clone().unwrap())
        }));
    }
    if user.is_some() {
        filters.push(Box::new(|offer: &Offer| -> bool {
            offer.is_owned(&user.clone().unwrap())
        }));
    }

    //ineffective, could be filtered in db query, or cached
    let offers: Vec<Offer> = db_queries::get_all_offers(&conn).unwrap();

    let filtered_offers: Vec<Offer> = offers
        .into_iter()
        .filter(|offer| filters.iter().all(|filter| filter(offer)))
        .collect();

    filtered_offers.iter().map(|o| o.as_json()).collect()
}

#[patch("/offers/<id>", format = "json", data = "<params>")]
pub fn offer_patch(conn: DbConn, cookies: Cookies, id: i32, params: String) -> Status {
    let user_mail = match authorize(&cookies, &conn) {
        Err(()) => return Status::Unauthorized,
        Ok(mail) => mail,
    };

    let mut offer = match db_queries::get_offer_by_id(&conn, id) {
        Ok(o) => o,
        Err(_) => return Status::NotFound,
    };

    if offer.owner.as_str() != user_mail.as_str() {
        return Status::Unauthorized;
    }

    let params_json = match json::parse(params.clone().as_str()) {
        Ok(j) => j,
        Err(_) => return Status::BadRequest,
    };

    if params_json["price"].as_f64().is_some() {
        offer.price = params_json["price"].as_f64().unwrap() as f32;
    }
    if params_json["description"].as_str().is_some() {
        offer.description = params_json["description"].as_str().unwrap().to_owned();
    }
    if params_json["amount"].as_i64().is_some() {
        if offer.type_.as_str() == "auction" {
            return Status::BadRequest;
        }
        offer.date_amount = params_json["amount"].as_i64().unwrap() as i32
    }

    match db_queries::update_offer(&conn, offer) {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[delete("/offers/<id>")]
pub fn offer_delete(conn: DbConn, cookies: Cookies, id: i32) -> Status {
    let user_mail = match authorize(&cookies, &conn) {
        Err(()) => return Status::Unauthorized,
        Ok(mail) => mail,
    };

    let offer = match db_queries::get_offer_by_id(&conn, id) {
        Ok(o) => o,
        Err(_) => return Status::NotFound,
    };

    if offer.owner.as_str() != user_mail.as_str() {
        return Status::Unauthorized;
    }

    match db_queries::offer_delete(&conn, id) {
        Ok(_) => Status::Accepted,
        Err(_) => Status::InternalServerError,
    }
}

#[get("/offers/<id>")]
pub fn offer_get(conn: DbConn, id: i32) -> Result<Custom<String>, Status> {
    let offer = match db_queries::get_offer_by_id(&conn, id) {
        Ok(o) => o,
        Err(_) => return Err(Status::NotFound),
    };

    if offer.type_.as_str() == "buynow" {
        let result_json: json::JsonValue = object!(
            "type" => offer.type_,
            "description" => offer.description,
            "price" => offer.price,
            "amount" => offer.date_amount
        );
        return Ok(Custom(Status::Ok, result_json.dump()));
    } else {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let status = if since_the_epoch.as_secs() > offer.date_amount as u64 {
            "expired"
        } else {
            "active"
        };
        match db_queries::get_transaction_by_offer_id(&conn, offer.id) {
            Ok(transaction) => {
                let result_json: json::JsonValue = object!(
                    "type" => offer.type_,
                    "description" => offer.description,
                    "status" => status,
                    "last_bid" => transaction.bid,
                    "customer_id" => transaction.buyer,
                    "expiration_ts" => offer.date_amount
                );
                return Ok(Custom(Status::Ok, result_json.dump()));
            }
            Err(diesel::result::Error::NotFound) => {
                let result_json: json::JsonValue = object!(
                    "type" => offer.type_,
                    "description" => offer.description,
                    "status" => status,
                    "last_bid" => offer.price,
                    "customer_id" => "",
                    "expiration_ts" => offer.date_amount
                );
                return Ok(Custom(Status::Ok, result_json.dump()));
            }
            Err(_) => return Err(Status::NotFound),
        };
    }
}

#[post("/offers/<id>/buy", format = "json", data = "<param>")]
pub fn offer_buy(
    conn: DbConn,
    param: String,
    id: i32,
    cookies: Cookies,
) -> Result<Custom<String>, Status> {
    let user_mail = match authorize(&cookies, &conn) {
        Err(()) => return Err(Status::Unauthorized),
        Ok(mail) => mail,
    };

    let param_json = match json::parse(param.as_str()) {
        Ok(j) => j,
        Err(_) => return Err(Status::BadRequest),
    };

    let mut offer = match db_queries::get_offer_by_id(&conn, id) {
        Ok(o) => o,
        Err(_) => return Err(Status::NotFound),
    };

    if offer.owner.as_str() == user_mail.as_str() {
        let response = object!(
            "conflict" => "unable to order own items"
        );
        return Ok(Custom(Status::Conflict, response.dump()));
    }

    if offer.type_.as_str() == "buynow" {
        if !param_json.has_key("amount") {
            return Err(Status::BadRequest);
        }
        let got_amount = match param_json["amount"].as_i32() {
            Some(a) => a,
            None => return Err(Status::BadRequest),
        };

        if got_amount > offer.date_amount {
            let response = object!(
                "max_amout" => offer.date_amount
            );
            return Ok(Custom(Status::Conflict, response.dump()));
        }

        let transaction = InsertableTransaction {
            offer_id: offer.id,
            buyer: user_mail.clone(),
            amount: Some(got_amount),
            bid: None,
        };
        match db_queries::insert_transaction(&conn, transaction) {
            Err(_) => return Err(Status::InternalServerError),
            Ok(_) => (),
        }
        offer.date_amount -= got_amount;
        match db_queries::update_offer(&conn, offer) {
            Err(_) => return Err(Status::InternalServerError),
            _ => (),
        }
    } else if offer.type_.as_str() == "auction" {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        if (offer.date_amount as u64) < since_the_epoch.as_secs() {
            let response = object!(
                "status" => "expired"
            );
            return Ok(Custom(Status::Conflict, response.dump()));
        }

        if !param_json.has_key("bid") {
            return Err(Status::BadRequest);
        }
        let got_bid = match param_json["bid"].as_f32() {
            Some(a) => a,
            None => return Err(Status::BadRequest),
        };

        match db_queries::get_transaction_by_offer_id(&conn, offer.id) {
            Ok(mut transaction) => {
                if transaction.bid.unwrap() >= got_bid {
                    let response = object! {
                        "minimal_bid" => transaction.bid
                    };
                    return Ok(Custom(Status::Conflict, response.dump()));
                }

                transaction.bid = Some(got_bid);
                transaction.buyer = user_mail;
                match db_queries::update_transaction(&conn, transaction) {
                    Err(_) => return Err(Status::InternalServerError),
                    _ => (),
                }
            }
            Err(diesel::result::Error::NotFound) => {
                if got_bid < offer.price {
                    let response = object! {
                        "minimal_bid" => offer.price
                    };
                    return Ok(Custom(Status::Conflict, response.dump()));
                }

                let transaction = InsertableTransaction {
                    offer_id: offer.id,
                    buyer: user_mail.clone(),
                    amount: None,
                    bid: Some(got_bid),
                };
                match db_queries::insert_transaction(&conn, transaction) {
                    Err(_) => return Err(Status::InternalServerError),
                    _ => (),
                };
            }
            _ => return Err(Status::InternalServerError),
        }
    }

    Err(Status::Ok)
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
