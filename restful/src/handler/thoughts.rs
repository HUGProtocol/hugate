use std::ptr::null;

use chrono::format;

use super::{
    user::{Address, UserInfoDetail},
    *,
};
use crate::handler::*;
use crate::{jwt::check_cookies, models::users};
use crate::{
    models::{comments, likes, thoughts},
    schema::comment,
};

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct GetPopularThoughtsListReq {
    #[form(field = "type")]
    pub thought_type: String,
    pub address: String,
    pub currentPage: i64,
    pub pageSize: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Thought {
    pub thought_id: i32,
    pub address: String,
    pub userInfo: String,
    pub tips: String,
    pub content: String,
    pub likeNum: i64,
    pub pic: String,
    #[serde(rename = "type")]
    pub thought_type: String,
    pub avatar: String,
    pub pts: i64,
    pub sourceUrl: String,
}

impl Default for Thought {
    fn default() -> Self {
        let default_user = UserInfoDetail::default();
        Self {
            thought_id: 1,
            address: default_user.address,
            userInfo: format!(
                "email: {} twitter {}",
                default_user.email, default_user.twitter
            ),
            tips: "First Image from NASA Webb Space Telescope".to_string(),
            content: "spaceship".to_string(),
            likeNum: 10,
            pic: "https://miro.medium.com/max/1400/1*OEnUxTYQaNxBSnXGpnpr5g.jpeg".to_string(),
            thought_type: "web".to_string(),
            avatar: default_user.profileImage,
            pts: 1000,
            sourceUrl: "https://medium.com/naaut/first-image-from-nasas-webb-5e691e5e16fc"
                .to_string(),
        }
    }
}

pub struct ThoughtsList(Vec<Thought>);

impl ThoughtsList {
    pub fn default(num: u32) -> Self {
        let v = (0..num)
            .map(|_| Thought::default())
            .collect::<Vec<Thought>>();
        ThoughtsList(v)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetPopularThoughtsListBody {
    pub thoughts: Vec<Thought>,
    pub total: u32,
}

impl GetPopularThoughtsListBody {
    fn default(num: u32) -> Self {
        Self {
            thoughts: ThoughtsList::default(num).0,
            total: num,
        }
    }
}

#[post("/getPopularThoughtsList", data = "<pop_thoughts_list_req>")]
pub fn get_popular_thoughts_list(
    conn: DbConn,
    pop_thoughts_list_req: LenientForm<GetPopularThoughtsListReq>,
) -> Json<HugResponse<Option<GetPopularThoughtsListBody>>> {
    let mut address = None;
    if pop_thoughts_list_req.address.len() != 0 {
        address = Some(pop_thoughts_list_req.address.clone());
    }
    let mut thought_type = None;
    if pop_thoughts_list_req.thought_type.len() != 0 {
        thought_type = Some(pop_thoughts_list_req.thought_type.clone());
    }
    let res = thoughts::Thoughts::get_popular(
        &conn,
        pop_thoughts_list_req.currentPage,
        pop_thoughts_list_req.pageSize,
        thought_type,
        address,
        None,
    );

    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "get popular thought failed".to_string(),
            resultBody: None,
        });
    }
    let (thought_db, _page_count) = res.unwrap();
    let cnt = thought_db.len();
    let mut though_list = vec![Thought::default(); cnt];
    let tt = though_list
        .into_iter()
        .zip(thought_db)
        .map(|(mut x, y)| {
            x.thought_id = y.id;
            x.address = y.address.clone();
            x.tips = y.tips.clone();
            x.content = y.content.clone();
            x.likeNum = y.likes;
            x.pic = y.snapshot.clone();
            x.thought_type = y.thought_type.clone();
            x.sourceUrl = y.source_url.clone();
            let res = users::Users::get_user_by_address(&conn, y.address.clone());
            if res.is_ok() {
                if let Some(us) = res.unwrap().get(0) {
                    x.avatar = us.profile_image.clone();
                    x.pts = us.pts;
                    x.userInfo = us.about.clone();
                }
            }
            x
        })
        .collect::<Vec<Thought>>();

    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(GetPopularThoughtsListBody {
            thoughts: tt,
            total: cnt as u32,
        }),
    })
}

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct GetMyThoughtsListReq {
    #[form(field = "type")]
    pub thought_type: String,
    pub address: String,
    pub state: String,
    pub viewed: String,
    pub currentPage: i64,
    pub pageSize: i64,
}

#[post("/getMyThoughtsList", data = "<my_thoughts_list_req>")]
pub fn get_my_thoughts_list(
    cookies: Cookies,
    conn: DbConn,
    my_thoughts_list_req: LenientForm<GetMyThoughtsListReq>,
) -> Json<HugResponse<Option<GetPopularThoughtsListBody>>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "check token failed".to_string(),
            resultBody: None,
        });
    }
    let role = res.unwrap();
    let mut address = role.address.clone();
    if my_thoughts_list_req.address.len() != 0 {
        address = my_thoughts_list_req.address.clone();
    }

    let res = thoughts::Thoughts::get_my(
        &conn,
        address.clone(),
        my_thoughts_list_req.currentPage,
        my_thoughts_list_req.pageSize,
        //todo:
        None,
        None,
    );

    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "get my thought failed".to_string(),
            resultBody: None,
        });
    }
    let (thought_db, _page_count) = res.unwrap();
    let cnt = thought_db.len();
    let mut though_list = vec![Thought::default(); cnt];
    let tt = though_list
        .into_iter()
        .zip(thought_db)
        .map(|(mut x, y)| {
            x.address = y.address.clone();
            x.tips = y.tips.clone();
            x.content = y.content.clone();
            x.likeNum = y.likes;
            x.pic = y.snapshot.clone();
            x.thought_type = y.thought_type.clone();
            x.sourceUrl = y.source_url.clone();
            x.thought_id = y.id;
            let res = users::Users::get_user_by_address(&conn, y.address.clone());
            if res.is_ok() {
                if let Some(us) = res.unwrap().get(0) {
                    x.avatar = us.profile_image.clone();
                    x.pts = us.pts;
                    x.userInfo = us.about.clone();
                }
            }
            x
        })
        .collect::<Vec<Thought>>();

    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(GetPopularThoughtsListBody {
            thoughts: tt,
            total: cnt as u32,
        }),
    })
}

#[derive(Serialize, Deserialize)]
pub struct ThoughtDetail {
    pub thought_id: i32,
    pub userName: String,
    pub tips: String,
    pub content: String,
    pub likeNum: i64,
    #[serde(rename = "type")]
    pub thought_type: String,
    pub avatar: String,
    pub pts: i64,
    pub sourceUrl: String,
    pub commentNum: i32,
    pub snapshot: String,
    pub if_like: i32,
}

impl Default for ThoughtDetail {
    fn default() -> Self {
        let u = UserInfoDetail::default();
        let t = Thought::default();
        Self {
            thought_id: 1,
            userName: u.userName,
            tips: t.tips,
            content: t.content,
            likeNum: t.likeNum as i64,
            thought_type: t.thought_type,
            avatar: u.profileImage,
            pts: t.pts as i64,
            sourceUrl: t.sourceUrl,
            commentNum: 100,
            snapshot: t.pic,
            if_like: 0,
        }
    }
}

#[get("/getThoughtDetail?<thoughtId>")]
pub fn get_thought_detail(
    cookies: Cookies,
    conn: DbConn,
    thoughtId: i32,
) -> Json<HugResponse<Option<ThoughtDetail>>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: format!("{}", res.err().unwrap().to_string()),
            resultBody: None,
        });
    }
    let role = res.unwrap();
    let mut thought_detail = ThoughtDetail::default();
    let res = thoughts::Thoughts::get_by_id(&conn, thoughtId);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "get thought failed".to_string(),
            resultBody: None,
        });
    }
    let r = res.unwrap();
    if r.len() == 0 {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: format!("no thought id {}", thoughtId),
            resultBody: None,
        });
    }

    let t = r.get(0).unwrap();
    thought_detail.tips = t.tips.clone();
    thought_detail.content = t.content.clone();
    thought_detail.thought_type = t.thought_type.clone();
    thought_detail.sourceUrl = t.source_url.clone();
    thought_detail.snapshot = t.snapshot.clone();
    thought_detail.likeNum = t.likes;
    thought_detail.thought_id = t.id;

    let res = users::Users::get_user_by_address(&conn, t.address.clone());
    if res.is_ok() {
        if let Some(u) = res.unwrap().get(0) {
            thought_detail.userName = u.username.clone();
            thought_detail.avatar = u.profile_image.clone();
            thought_detail.pts = u.pts;
        }
    }

    let res = likes::Likes::if_like(
        &conn,
        likes::NewLike {
            address: role.address.clone(),
            thought_id: thoughtId,
        },
    );
    if res {
        thought_detail.if_like = 1;
    } else {
        thought_detail.if_like = 0;
    }

    let res = comments::Comment::get_count_by_thought_id(&conn, t.id);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        thought_detail.commentNum = res.unwrap() as i32;
    }

    return Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(thought_detail),
    });
}

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct LikeOrUnlikeThoughtReq {
    pub thoughtId: i32,
    pub operate: i32,
}
#[post("/likeOrUnlikeThought", data = "<like_req>")]
pub fn like_or_unlike_thought(
    cookies: Cookies,
    conn: DbConn,
    like_req: LenientForm<LikeOrUnlikeThoughtReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    if like_req.operate == 1 {
        let res = likes::Likes::like(
            &conn,
            likes::NewLike {
                address: role.address,
                thought_id: like_req.thoughtId,
            },
        );
        if res {
            thoughts::Thoughts::add_like(&conn, like_req.thoughtId);
            return Json(HugResponse::new_success());
        } else {
            return Json(HugResponse::new_failed("like failed", ""));
        }
    }
    if like_req.operate == 0 {
        let res = likes::Likes::unlike(
            &conn,
            likes::NewLike {
                address: role.address,
                thought_id: like_req.thoughtId,
            },
        );
        thoughts::Thoughts::reduce_unlike(&conn, like_req.thoughtId);
        if res {
            return Json(HugResponse::new_success());
        } else {
            return Json(HugResponse::new_failed("unlike failed", ""));
        }
    }

    Json(HugResponse::new_failed("invalid operate code", ""))
}

#[derive(FromForm)]
pub struct RewardReq {
    pub ptsNum: u32,
    pub thoughtId: i32,
}

#[post("/reward", data = "<reward_req>")]
pub fn reward(
    db_conn: DbConn,
    reward_req: Form<RewardReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
pub struct CreateThoughtReq {
    sourceUrl: String,
    sourceIco: String,
    snapshot: String,
    #[form(field = "Thoughts")]
    thoughts_content: String,
    tips: String,
    viewed: String,
    submitState: String,
    #[form(field = "type")]
    thought_type: String,
}
#[post("/createThoughts", data = "<req>")]
pub fn createThoughts(
    cookies: Cookies,
    conn: DbConn,
    req: Form<CreateThoughtReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    let res = thoughts::Thoughts::create(
        &conn,
        thoughts::NewThought {
            content: req.thoughts_content.clone(),
            address: address.clone(),
            tips: req.tips.clone(),
            thought_type: req.thought_type.clone(),
            source_url: req.sourceUrl.clone(),
            snapshot: req.snapshot.clone(),
            viewed: req.viewed.clone(),
            submit_state: req.submitState.clone(),
        },
    );
    if res == false {
        return Json(HugResponse::new_failed("create thought failed", ""));
    }
    Json(HugResponse::new_success())
}
