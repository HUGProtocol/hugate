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
use curl::easy::Easy;

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct GetPopularThoughtsListReq {
    #[form(field = "type")]
    pub thought_type: String,
    pub address: String,
    pub order_by: Option<i32>,
    pub currentPage: i64,
    pub pageSize: i64,
}

#[derive(Serialize, Clone)]
pub struct Thought {
    pub thought_id: i32,
    pub address: String,
    pub userInfo: users::Users,
    pub tips: String,
    pub content: String,
    pub likeNum: i64,
    pub pic: String,
    #[serde(rename = "type")]
    pub thought_type: String,
    pub avatar: String,
    pub pts: i64,
    pub comment_num: i64,
    pub sourceUrl: String,
    pub embeded: String,
    pub create_time: i64,
    pub state: String,
    #[serde(rename = "tokenId")]
    pub pass_token_id: i64,
    pub viewed: String,
    pub thought_nft_id: i64,
}

impl Default for Thought {
    fn default() -> Self {
        let default_user = UserInfoDetail::default();
        Self {
            thought_id: 1,
            address: default_user.address,
            // userInfo: users::Users { id: 12, username: "default".to_string(), profile_image: "default".to_string(), email: "default".to_string(), twitter: "default".to_string(), about: "default".to_string(), pts: 0, create_at: NaiveDateTime::default(), updated_at: NaiveDateTime::default(), address: "default".to_string() },
            userInfo: users::Users::default(),
            tips: "First Image from NASA Webb Space Telescope".to_string(),
            content: "spaceship".to_string(),
            likeNum: 10,
            pic: "https://miro.medium.com/max/1400/1*OEnUxTYQaNxBSnXGpnpr5g.jpeg".to_string(),
            thought_type: "web".to_string(),
            avatar: default_user.profileImage,
            pts: 0,
            sourceUrl: "https://medium.com/naaut/first-image-from-nasas-webb-5e691e5e16fc"
                .to_string(),
            comment_num: 0,
            embeded: "".to_string(),
            create_time: 0,
            state: "".to_string(),
            pass_token_id: -1,
            viewed: "".to_string(),
            thought_nft_id: -1,
            // embeded:r#"<blockquote class="twitter-tweet"><p lang="en" dir="ltr">It was a magical evening yesterday. Thank you again to all the players and fans who were here to share this moment with me. It means the world ❤️😊🙏🏼 <a href="https://t.co/IKFb6jEeXJ">pic.twitter.com/IKFb6jEeXJ</a></p>&mdash; Roger Federer (@rogerfederer) <a href="https://twitter.com/rogerfederer/status/1573632451632570369?ref_src=twsrc%5Etfw">September 24, 2022</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>"#.to_string(),
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

#[derive(Serialize)]
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
        pop_thoughts_list_req.order_by,
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
            x.pts = y.pts;
            x.embeded = y.embeded.clone();
            x.create_time = y.create_at.timestamp();
            x.state = y.submit_state.clone();
            x.viewed = y.viewed.clone();
            x.thought_nft_id = y.token_id;
            if y.viewed == "pass" {
                let res = pass::Pass::get_by_thought(&conn, y.id as i64);
                if let Ok(list) = res {
                    if list.len() > 0 {
                        x.pass_token_id = list.get(0).unwrap().token_id;
                    }
                } else {
                    println!(
                        "{}",
                        format!("viewed value pass but tokenId not fount {}", y.id)
                    );
                    x.viewed = "all".to_string();
                }
            }
            let res = users::Users::get_user_by_address(&conn, y.address.clone());
            if res.is_ok() {
                if let Some(us) = res.unwrap().get(0) {
                    x.userInfo = us.clone();
                }
            }
            let res = comments::Comment::get_count_by_thought_id(&conn, y.id);
            if res.is_ok() {
                x.comment_num = res.unwrap();
            }
            x
        })
        .collect::<Vec<Thought>>();

    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(GetPopularThoughtsListBody {
            thoughts: tt,
            total: _page_count as u32,
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

    let mut thought_type = None;
    if my_thoughts_list_req.thought_type != "" {
        thought_type = Some(my_thoughts_list_req.thought_type.to_owned());
    }

    let mut viewed = None;
    if my_thoughts_list_req.viewed != "" {
        viewed = Some(my_thoughts_list_req.viewed.to_owned());
    }
    let mut submit_state = None;
    if my_thoughts_list_req.state != "" {
        submit_state = Some(my_thoughts_list_req.state.clone());
    }

    let res = {
        if submit_state == Some("like".to_string()) {
            thoughts::Thoughts::get_my_like(
                &conn,
                address.clone(),
                my_thoughts_list_req.currentPage,
                my_thoughts_list_req.pageSize,
                thought_type,
                viewed,
            )
        } else {
            thoughts::Thoughts::get_my(
                &conn,
                address.clone(),
                my_thoughts_list_req.currentPage,
                my_thoughts_list_req.pageSize,
                thought_type,
                viewed,
                submit_state,
            )
        }
    };

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
            x.pts = y.pts;
            x.embeded = y.embeded.clone();
            x.create_time = y.create_at.timestamp();
            x.state = y.submit_state.clone();
            x.viewed = y.viewed.clone();
            x.thought_nft_id = y.token_id;
            if y.viewed == "pass" {
                let res = pass::Pass::get_by_thought(&conn, y.id as i64);
                if let Ok(list) = res {
                    if list.len() > 0 {
                        x.pass_token_id = list.get(0).unwrap().token_id;
                    }
                } else {
                    println!(
                        "{}",
                        format!("viewed value pass but tokenId not fount {}", y.id)
                    );
                    // x.viewed = "all".to_string();
                }
            }
            let res = users::Users::get_user_by_address(&conn, y.address.clone());
            if res.is_ok() {
                if let Some(us) = res.unwrap().get(0) {
                    x.userInfo = us.clone()
                }
            }
            let res = comments::Comment::get_count_by_thought_id(&conn, y.id);
            if res.is_ok() {
                x.comment_num = res.unwrap();
            }
            x
        })
        .collect::<Vec<Thought>>();

    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(GetPopularThoughtsListBody {
            thoughts: tt,
            total: _page_count as u32,
        }),
    })
}

#[derive(Serialize)]
pub struct ThoughtDetail {
    pub thought_id: i32,
    pub userName: String,
    pub userInfo: users::Users,
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
    pub html: String,
    pub embeded: String,
    pub create_time: i64,
    pub html_backup: String,
    #[serde(rename = "tokenId")]
    pub pass_token_id: i64,
    pub thought_nft_id: i64,
    pub viewed: String,
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
            pts: 0,
            sourceUrl: t.sourceUrl,
            commentNum: 100,
            snapshot: t.pic,
            if_like: 0,
            userInfo: users::Users::default(),
            html: "".to_string(),
            embeded: "".to_string(),
            create_time: 0,
            html_backup: "".to_string(),
            pass_token_id: -1,
            viewed: "".to_string(),
            thought_nft_id: -1,
            // embeded: r#"<blockquote class="twitter-tweet"><p lang="en" dir="ltr">It was a magical evening yesterday. Thank you again to all the players and fans who were here to share this moment with me. It means the world ❤️😊🙏🏼 <a href="https://t.co/IKFb6jEeXJ">pic.twitter.com/IKFb6jEeXJ</a></p>&mdash; Roger Federer (@rogerfederer) <a href="https://twitter.com/rogerfederer/status/1573632451632570369?ref_src=twsrc%5Etfw">September 24, 2022</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>"#.to_string(),
        }
    }
}

pub fn get_thought_detail_by_id(
    conn: &PgConnection,
    thoughtId: i32,
    jwt_addr: Option<String>,
) -> Option<ThoughtDetail> {
    let mut thought_detail = ThoughtDetail::default();
    let r = thoughts::Thoughts::get_by_id(conn, thoughtId).ok()?;
    if r.len() == 0 {
        return None;
    }
    let t = r.get(0).unwrap();
    thought_detail.tips = t.tips.clone();
    thought_detail.content = t.content.clone();
    thought_detail.thought_type = t.thought_type.clone();
    thought_detail.sourceUrl = t.source_url.clone();
    thought_detail.snapshot = t.snapshot.clone();
    thought_detail.likeNum = t.likes;
    thought_detail.thought_id = t.id;
    thought_detail.html = t.html.clone();
    thought_detail.pts = t.pts;
    thought_detail.embeded = t.embeded.clone();
    thought_detail.create_time = t.create_at.timestamp();
    thought_detail.html_backup = t.html_backup.clone();
    thought_detail.viewed = t.viewed.clone();
    thought_detail.thought_nft_id = t.token_id;

    let res = users::Users::get_user_by_address(conn, t.address.clone());
    if res.is_ok() {
        if let Some(u) = res.unwrap().get(0) {
            thought_detail.userName = u.username.clone();
            thought_detail.avatar = u.profile_image.clone();
            thought_detail.userInfo = u.clone();
        }
    }

    let mut res = false;
    if let Some(addr) = jwt_addr.clone() {
        res = likes::Likes::if_like(
            &conn,
            likes::NewLike {
                address: addr,
                thought_id: thoughtId,
            },
        );
    }

    if res {
        thought_detail.if_like = 1;
    } else {
        thought_detail.if_like = 0;
    }

    let res = comments::Comment::get_count_by_thought_id(conn, t.id);
    if let Err(e) = res {
        println!("{}", e);
    } else {
        thought_detail.commentNum = res.unwrap() as i32;
    }

    Some(thought_detail)
}

#[get("/getThoughtDetail?<thoughtId>")]
pub fn get_thought_detail(
    cookies: Cookies,
    conn: DbConn,
    thoughtId: i32,
) -> Json<HugResponse<Option<ThoughtDetail>>> {
    let jwt_res = check_cookies(&cookies);
    let mut jwt_addr = None;
    if jwt_res.is_ok() {
        let role = jwt_res.unwrap();
        jwt_addr = Some(role.address.clone());
    }

    // if res.is_err() {
    //     return Json(HugResponse {
    //         resultCode: 500,
    //         resultMsg: format!("{}", res.err().unwrap().to_string()),
    //         resultBody: None,
    //     });
    // }
    // let role = res.unwrap();
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
    if t.viewed == "followers" {
        if jwt_addr.is_none() {
            return Json(HugResponse {
                resultCode: 500,
                resultMsg: format!("{}", "token check failed"),
                resultBody: None,
            });
        }
        if t.address != jwt_addr.clone().unwrap() {
            let is_follow =
                users::Users::if_follow(jwt_addr.clone().unwrap(), t.address.clone(), &conn);
            if !is_follow {
                return Json(HugResponse {
                    resultCode: 500,
                    resultMsg: format!("only follower"),
                    resultBody: None,
                });
            }
        }
    }
    thought_detail.tips = t.tips.clone();
    thought_detail.content = t.content.clone();
    thought_detail.thought_type = t.thought_type.clone();
    thought_detail.sourceUrl = t.source_url.clone();
    thought_detail.snapshot = t.snapshot.clone();
    thought_detail.likeNum = t.likes;
    thought_detail.thought_id = t.id;
    thought_detail.html = t.html.clone();
    thought_detail.pts = t.pts;
    thought_detail.embeded = t.embeded.clone();
    thought_detail.create_time = t.create_at.timestamp();
    thought_detail.html_backup = t.html_backup.clone();
    thought_detail.viewed = t.viewed.clone();
    thought_detail.thought_nft_id = t.token_id;
    if t.viewed == "pass" {
        if jwt_addr.is_none() {
            return Json(HugResponse {
                resultCode: 500,
                resultMsg: format!("{}", "token check failed"),
                resultBody: None,
            });
        }
        let res = pass::Pass::get_by_thought(&conn, t.id as i64);
        if let Ok(pass_vec) = res {
            if let Some(ps) = pass_vec.first() {
                let pass_token_id = ps.token_id;
                thought_detail.pass_token_id = pass_token_id;
                if t.address != jwt_addr.clone().unwrap() {
                    let pass_cnt =
                        check_pass_balance(jwt_addr.clone().unwrap(), pass_token_id as i32);
                    match pass_cnt {
                        None => {
                            return Json(HugResponse {
                                resultCode: 500,
                                resultMsg: format!(
                                    "{} token_id:{}",
                                    "check pass failed", pass_token_id
                                ),
                                resultBody: None,
                            });
                        }
                        Some(cnt) => {
                            if cnt == 0 {
                                return Json(HugResponse {
                                    resultCode: 500,
                                    resultMsg: format!("{} token_id:{}", "no pass", pass_token_id),
                                    resultBody: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    let res = users::Users::get_user_by_address(&conn, t.address.clone());
    if res.is_ok() {
        if let Some(u) = res.unwrap().get(0) {
            thought_detail.userName = u.username.clone();
            thought_detail.avatar = u.profile_image.clone();
            thought_detail.userInfo = u.clone();
        }
    }

    let mut res = false;
    if let Some(addr) = jwt_addr.clone() {
        res = likes::Likes::if_like(
            &conn,
            likes::NewLike {
                address: addr,
                thought_id: thoughtId,
            },
        );
    }

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
    cookies: Cookies,
    conn: DbConn,
    reward_req: Form<RewardReq>,
) -> Json<HugResponse<Option<OneLineResultBody>>> {
    //check cookies
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
    let res = models::users::Users::get_user_by_address(&conn, address.clone());
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "user not found".to_string(),
            resultBody: None,
        });
    }
    if let Ok(ref user_list) = res {
        if user_list.is_empty() {
            return Json(HugResponse {
                resultCode: 500,
                resultMsg: "user not found".to_string(),
                resultBody: None,
            });
        }
    }
    let user_info = res.unwrap().get(0).unwrap().clone();
    if user_info.pts < reward_req.ptsNum as i64 {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "pts not enough".to_string(),
            resultBody: None,
        });
    }
    let res = thoughts::Thoughts::get_by_id(&conn, reward_req.thoughtId);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: format!("no such thought id {}", reward_req.thoughtId).to_string(),
            resultBody: None,
        });
    }
    if let Ok(list) = res {
        if list.is_empty() {
            return Json(HugResponse {
                resultCode: 500,
                resultMsg: format!("no such thought id {}", reward_req.thoughtId).to_string(),
                resultBody: None,
            });
        }
        let thought = list.get(0).unwrap();
        thoughts::Thoughts::update_pts(
            &conn,
            reward_req.thoughtId,
            thought.pts + reward_req.ptsNum as i64,
        );
        users::Users::reduce_pts(address.clone(), &conn, reward_req.ptsNum as i64);
        users::Users::add_pts(thought.address.clone(), &conn, reward_req.ptsNum as i64);
    }
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
pub struct ChangeStateReq {
    thought_id: i32,
    submit_state_op: Option<String>,
    viewed_op: Option<String>,
    thought_id_op: Option<i64>,
    pass_id_op: Option<i64>,
}

#[post("/changeThoughtState", data = "<req>")]
pub fn changeThoughtState(
    cookies: Cookies,
    conn: DbConn,
    req: Form<ChangeStateReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    let res = thoughts::Thoughts::get_by_id(&conn, req.thought_id);
    if res.is_err() {
        return Json(HugResponse::new_failed("no such thought id", ""));
    }

    match res.unwrap().get(0) {
        None => return Json(HugResponse::new_failed("no such thought id", "")),
        Some(t) => {
            if t.address != address {
                return Json(HugResponse::new_failed("not thought owner", ""));
            }
            if let Some(new_state) = req.submit_state_op.clone() {
                if new_state != "" {
                    let ok = thoughts::Thoughts::update_state(&conn, new_state, req.thought_id);
                    if !ok {
                        return Json(HugResponse::new_failed("update state failed", ""));
                    }
                }
            }

            if let Some(new_viewed) = req.viewed_op.clone() {
                if new_viewed != "" {
                    let ok = thoughts::Thoughts::update_viewed(&conn, new_viewed, req.thought_id);
                    if !ok {
                        return Json(HugResponse::new_failed("update viewed failed", ""));
                    }
                }
            }

            if let Some(new_thought_id) = req.thought_id_op {
                if new_thought_id > 0 {
                    let ok =
                        thoughts::Thoughts::update_token_id(&conn, new_thought_id, req.thought_id);
                    if !ok {
                        return Json(HugResponse::new_failed("update thought id failed", ""));
                    }
                }
            }

            if let Some(new_pass_id) = req.pass_id_op {
                if new_pass_id > 0 {
                    let ok = pass::Pass::put_pass(
                        &conn,
                        pass::NewPass {
                            thought_id: req.thought_id as i64,
                            token_id: new_pass_id,
                        },
                    );
                    if !ok {
                        return Json(HugResponse::new_failed("update pass token id failed", ""));
                    }
                }
            }
        }
    }
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
    html: String,
    thought_id_op: Option<i32>,
    html_backup: String,
    token_id_op: Option<i64>,
    thought_nft_id: Option<i64>,
}
#[post("/createThoughts", data = "<req>")]
pub fn createThoughts(
    cookies: Cookies,
    conn: DbConn,
    req: Form<CreateThoughtReq>,
) -> Json<HugResponse<Option<ThoughtDetail>>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_none("check token failed"));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    let mut new_thought = thoughts::NewThought {
        content: req.thoughts_content.clone(),
        address: address.clone(),
        tips: req.tips.clone(),
        thought_type: req.thought_type.clone(),
        source_url: req.sourceUrl.clone(),
        snapshot: req.snapshot.clone(),
        viewed: req.viewed.clone(),
        submit_state: req.submitState.clone(),
        html: req.html.clone(),
        embeded: "".to_string(),
        html_backup: req.html_backup.clone(),
        token_id: -1,
    };
    if let Some(thought_id) = req.thought_id_op {
        if thought_id != 0 {
            let res = thoughts::Thoughts::get_by_id(&conn, thought_id);
            if res.is_err() {
                return Json(HugResponse::new_none("no such thought"));
            }

            match res.unwrap().get(0) {
                None => return Json(HugResponse::new_none("no such thought")),
                Some(t) => {
                    if t.address != address {
                        return Json(HugResponse::new_none("not thought owner"));
                    }
                    if let Some(thought_token_id) = req.thought_nft_id {
                        if thought_token_id > 0 {
                            new_thought.token_id = thought_token_id;
                        }
                    }
                    if t.source_url != new_thought.source_url || t.embeded == "" {
                        if new_thought.source_url.contains("twitter") {
                            if let Some(embeded) = curl_twitter(new_thought.source_url.clone()) {
                                if let Ok(s) = std::str::from_utf8(&embeded) {
                                    new_thought.embeded = s.to_string();
                                }
                            }
                        }
                    }
                    thoughts::Thoughts::update(&conn, new_thought, thought_id);
                    if let Some(pass_token_id) = req.token_id_op {
                        if pass_token_id >= 0 {
                            pass::Pass::put_pass(
                                &conn,
                                pass::NewPass {
                                    thought_id: thought_id as i64,
                                    token_id: pass_token_id,
                                },
                            );
                        }
                    }
                }
            }
            let detail = get_thought_detail_by_id(&conn, thought_id, Some(address));
            return Json(HugResponse {
                resultCode: 200,
                resultMsg: "success".to_string(),
                resultBody: detail,
            });
        }
    }
    if new_thought.source_url.contains("twitter") {
        if let Some(embeded) = curl_twitter(new_thought.source_url.clone()) {
            if let Ok(s) = std::str::from_utf8(&embeded) {
                new_thought.embeded = s.to_string();
            }
        }
    }
    let res = thoughts::Thoughts::create(&conn, new_thought);
    if res.is_err() {
        return Json(HugResponse::new_none("create thought failed"));
    }
    let thought_id = res.unwrap();

    if let Some(pass_token_id) = req.token_id_op {
        if pass_token_id >= 0 {
            pass::Pass::put_pass(
                &conn,
                pass::NewPass {
                    thought_id: thought_id as i64,
                    token_id: pass_token_id,
                },
            );
        }
    }

    let detail = get_thought_detail_by_id(&conn, thought_id, Some(address));
    return Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: detail,
    });
}

#[get("/getPassTokenId?<thoughtId>")]
pub fn getPassTokenId(conn: DbConn, thoughtId: i64) -> Json<HugResponse<OneLineResultBody>> {
    let res = pass::Pass::get_by_thought(&conn, thoughtId);
    match res {
        Err(_) => {
            return Json(HugResponse::new_success());
        }
        Ok(list) => {
            if list.len() == 0 {
                return Json(HugResponse::new_success());
            }
            let token_list = list.into_iter().map(|x| x.token_id).collect::<Vec<i64>>();

            return Json(HugResponse {
                resultCode: 200,
                resultMsg: "success".to_string(),
                resultBody: OneLineResultBody(
                    format!("{}", token_list.get(0).unwrap()).to_string(),
                ),
            });
        }
    }
}

#[get("/getPassThoughtId?<tokenId>")]
pub fn getPassThoughtId(conn: DbConn, tokenId: i64) -> Json<HugResponse<OneLineResultBody>> {
    let res = pass::Pass::get_by_token(&conn, tokenId);
    match res {
        Err(_) => {
            return Json(HugResponse::new_success());
        }
        Ok(list) => {
            if list.len() == 0 {
                return Json(HugResponse::new_success());
            }
            let thought_list = list.into_iter().map(|x| x.thought_id).collect::<Vec<i64>>();

            return Json(HugResponse {
                resultCode: 200,
                resultMsg: "success".to_string(),
                resultBody: OneLineResultBody(format!("{:?}", thought_list).to_string()),
            });
        }
    }
}

#[derive(FromForm)]
pub struct EmbededCardReq {
    sourceUrl: String,
}
#[post("/embededCard", data = "<req>")]
pub fn embededCard(req: Form<EmbededCardReq>) -> Json<HugResponse<OneLineResultBody>> {
    if !req.sourceUrl.contains("twitter") {
        return Json(HugResponse::new_failed("not twitter url", ""));
    }
    if let Some(embeded) = curl_twitter(req.sourceUrl.clone()) {
        if let Ok(s) = std::str::from_utf8(&embeded) {
            return Json(HugResponse {
                resultCode: 200,
                resultMsg: "success".to_string(),
                resultBody: OneLineResultBody(s.to_string()),
            });
        }
    }
    Json(HugResponse::new_failed(
        "get twitter embeded failed",
        &req.sourceUrl,
    ))
}

pub fn curl_twitter(url: String) -> Option<Vec<u8>> {
    let mut embeded_url = r#"https://publish.twitter.com/oembed?url="#.to_string();
    embeded_url += &url;
    let mut easy = Easy::new();
    let res = easy.url(&embeded_url);
    if res.is_err() {
        return None;
    }
    let mut dst = Vec::new();
    {
        let mut transfer = easy.transfer();
        let res = transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        });
        if res.is_err() {
            return None;
        }
        let res = transfer.perform();
        if res.is_err() {
            return None;
        }
    }
    Some(dst)
}

use diesel::PgConnection;
use hex_literal::hex;
use tokio::runtime::Runtime;
use web3::contract::{tokens::Tokenizable, Contract, Options};
use web3::types::U256;
pub fn check_pass_balance(address: String, pass_token_id: i32) -> Option<u32> {
    let http_url = "https://avalanche-fuji.infura.io/v3/ce421f619bc34c37a0fb86075d41226f";
    let http = web3::transports::Http::new(http_url).ok()?;
    let web3 = web3::Web3::new(http);
    let contract_address = hex!("d29C5baBfb1E382Cc1e0a7E575A4a45a1bAaA64F").into();
    if address.len() < 3 {
        return None;
    }
    let hex_address = &address.as_str()[2..];
    let decoded_string = hex::decode(hex_address).ok()?;
    let aa: [u8; 20] = decoded_string.as_slice()[0..20].try_into().ok()?;
    let user_address: web3::types::Address = aa.into();
    let abi_json = r#"[
            {
              "inputs": [
                {
                  "internalType": "address",
                  "name": "account",
                  "type": "address"
                },
                {
                  "internalType": "uint256",
                  "name": "id",
                  "type": "uint256"
                }
              ],
              "name": "balanceOf",
              "outputs": [
                {
                  "internalType": "uint256",
                  "name": "",
                  "type": "uint256"
                }
              ],
              "stateMutability": "view",
              "type": "function"
            }
          ]
          "#
    .as_bytes();
    let contract = Contract::from_json(web3.eth(), contract_address, abi_json).ok()?;
    let tokenid = U256::from(pass_token_id);
    let fut = contract.query(
        "balanceOf",
        (user_address, tokenid),
        None,
        Options::default(),
        None,
    );
    let runtime = Runtime::new().ok()?;
    let v: U256 = runtime.block_on(fut).ok()?;
    Some(v.as_u32())
}
