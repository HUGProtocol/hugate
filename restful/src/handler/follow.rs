use super::{user::UserInfoDetail, *};

#[derive(FromForm)]
pub struct FollowOrNotReq {
    pub address: String,
    pub operate: i32,
}

#[post("/followOrNot", data = "<follow_or_not_req>")]
pub fn follow_or_not(
    db_conn: DbConn,
    follow_or_not_req: Form<FollowOrNotReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::<OneLineResultBody>::new_success())
}

#[derive(Serialize, Deserialize)]
pub struct getFollowListBody {
    pub follows: Vec<UserInfoDetail>,
    pub total: u32,
}

impl getFollowListBody {
    pub fn default(num: u32) -> Self {
        let followers = (0..num)
            .map(|_| UserInfoDetail::random())
            .collect::<Vec<UserInfoDetail>>();
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
    db_conn: DbConn,
    req: Option<Form<get_follow_list_req>>,
) -> Json<HugResponse<getFollowListBody>> {
    let num = req.map_or(5, |r| r.follow_type as u32);
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: getFollowListBody::default(num),
    })
}