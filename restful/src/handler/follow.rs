use crate::{
    jwt::check_cookies,
    models::users::{NewFollow, Users},
};

use super::{
    user::{UserInfoAbstract, UserInfoDetail},
    *,
};

#[derive(FromForm)]
pub struct FollowOrNotReq {
    pub address: String,
    pub operate: i32,
}

#[post("/followOrNot", data = "<follow_or_not_req>")]
pub fn follow_or_not(
    cookies: Cookies,
    conn: DbConn,
    follow_or_not_req: Form<FollowOrNotReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("token check failed", ""));
    }
    let role = res.unwrap();

    if follow_or_not_req.operate != 0 && follow_or_not_req.operate != 1 {
        return Json(HugResponse::new_failed("operation code incorrect", ""));
    }
    if follow_or_not_req.operate == 1 {
        let res = Users::follow(
            NewFollow {
                followee: follow_or_not_req.address.clone(),
                follower: role.address,
            },
            &conn,
        );
        if !res {
            return Json(HugResponse::new_failed("follow failed", ""));
        }
        return Json(HugResponse::<OneLineResultBody>::new_success());
    }

    let res = Users::unfollow(
        NewFollow {
            followee: follow_or_not_req.address.clone(),
            follower: role.address,
        },
        &conn,
    );
    if !res {
        return Json(HugResponse::new_failed("unfollow failed", ""));
    }
    return Json(HugResponse::<OneLineResultBody>::new_success());
}

#[derive(Serialize, Deserialize)]
pub struct getFollowListBody {
    pub follows: Vec<UserInfoAbstract>,
    pub total: u32,
}

impl getFollowListBody {
    pub fn default(num: u32) -> Self {
        let followers = (0..num)
            .map(|_| UserInfoAbstract::random())
            .collect::<Vec<UserInfoAbstract>>();
        getFollowListBody {
            follows: followers,
            total: num as u32,
        }
    }
}

#[derive(FromForm)]
pub struct get_follow_list_req {
    #[form(field = "type")]
    pub follow_type: i32,
}

#[get("/getFollowList?<req..>")]
pub fn get_follow_list(
    cookies: Cookies,
    conn: DbConn,
    req: Option<Form<get_follow_list_req>>,
) -> Json<HugResponse<getFollowListBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "check token failed".to_string(),
            resultBody: getFollowListBody::default(0),
        });
    }
    let role = res.unwrap();
    if req.is_none() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "not set type".to_string(),
            resultBody: getFollowListBody::default(0),
        });
    }

    let typep = req.unwrap().follow_type;
    if typep != 0 && typep != 1 {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "type incorrect".to_string(),
            resultBody: getFollowListBody::default(0),
        });
    }
    if typep == 0 {
        let res = Users::get_followers(&conn, role.address.clone());
        if res.is_err() {
            return Json(HugResponse {
                resultCode: 500,
                resultMsg: "get follower failed".to_string(),
                resultBody: getFollowListBody::default(0),
            });
        }
        let followers = res.unwrap();
        let count = followers.len();
        return Json(HugResponse {
            resultCode: 200,
            resultMsg: "success".to_string(),
            resultBody: getFollowListBody {
                follows: followers,
                total: count as u32,
            },
        });
    }

    let res = Users::get_followees(&conn, role.address.clone());
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "get follower failed".to_string(),
            resultBody: getFollowListBody::default(0),
        });
    }
    let followee = res.unwrap();
    let count = followee.len();
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: getFollowListBody {
            follows: followee,
            total: count as u32,
        },
    })
}
