use std::{fmt::format, ptr::null};

use chrono::{format, NaiveDate, NaiveDateTime};

use super::{
    user::{Address, UserInfoDetail},
    *,
};
use crate::{handler::*, models};
use crate::{jwt::check_cookies, models::users};
use crate::{
    models::{comments, likes, pass, thoughts},
    schema::pass::token_id,
};

#[derive(Serialize)]
pub struct metadata_list_body {
    pub metadata: Vec<pass::Metadata>,
    pub total: i32,
}

#[post("/createMetadata", data = "<req>")]
pub fn createMetadata(
    cookies: Cookies,
    conn: DbConn,
    req: Form<pass::Metadata>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    // if address != req.address {
    //     return Json(HugResponse::new_failed("address not correct", ""));
    // }
    let new_metadata = pass::NewMetadata {
        meta_json: req.meta_json.clone(),
        address: address.clone(),
        token_id: req.token_id,
    };
    //update
    if req.id > 0 {
        let res = pass::Metadata::update(&conn, &new_metadata, req.id);
        if !res {
            return Json(HugResponse::new_failed(
                format!("update {} failed", req.id).as_str(),
                "",
            ));
        }
        return Json(HugResponse::new_success());
    }
    //create
    let res = pass::Metadata::new(&conn, &new_metadata);
    if !res {
        return Json(HugResponse::new_failed("create failed", ""));
    }
    Json(HugResponse::new_success())
}

#[get("/getMetadataByAddress")]
pub fn get_metadata_by_address(
    cookies: Cookies,
    conn: DbConn,
) -> Json<HugResponse<Option<metadata_list_body>>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_none("check token failed"));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    let res = pass::Metadata::get_by_address(&conn, address);

    if res.is_ok() {
        let list = res.unwrap();
        let total = list.len() as i32;
        return Json(HugResponse {
            resultCode: 200,
            resultMsg: "success".to_string(),
            resultBody: Some(metadata_list_body {
                metadata: list,
                total: total,
            }),
        });
    }
    Json(HugResponse::new_none("check token failed"))
}
