pub mod comments;
pub mod follow;
pub mod medal;
pub mod metadata;
pub mod thoughts;
pub mod user;

use super::db::Conn as DbConn;

use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, LenientForm};
use rocket::Request;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct HugResponse<T: Serialize> {
    pub resultCode: i32,
    pub resultMsg: String,
    pub resultBody: T,
}

impl<T: Serialize + Default> HugResponse<T> {
    pub fn new_success() -> Self {
        HugResponse {
            resultCode: 200,
            resultMsg: "success".to_string(),
            resultBody: T::default(),
        }
    }
}

impl<T: Serialize> HugResponse<Option<T>> {
    pub fn new_none(show_error: &str) -> Self {
        HugResponse {
            resultCode: 500,
            resultMsg: show_error.to_string(),
            resultBody: None,
        }
    }
}

impl HugResponse<OneLineResultBody> {
    pub fn new_failed(show_error: &str, detail_err: &str) -> Self {
        HugResponse {
            resultCode: 500,
            resultMsg: show_error.to_string(),
            resultBody: OneLineResultBody(detail_err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct OneLineResultBody(String);

#[catch(404)]
pub fn not_found(req: &Request) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_failed(
        format!("Sorry, '{}' is not a valid path.", req.uri()).as_str(),
        "",
    ))
}

#[catch(422)]
pub fn miss_variable(req: &Request) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_failed(
        format!("Sorry, request to '{}' miss variable", req.uri()).as_str(),
        "",
    ))
}
