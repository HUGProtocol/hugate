use super::{
    user::{Address, UserInfoDetail, UserName},
    *,
};
use crate::models::{likes, users};
use crate::{jwt::check_cookies, models::comments};

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub address: String,
    pub userName: String,
    pub likeNum: i64,
    pub avatar: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct getThoughtsCommentsBody {
    pub total: u32,
    pub if_like: i32,
    pub comments: Vec<Comment>,
}

impl getThoughtsCommentsBody {
    pub fn default(num: u32) -> Self {
        let u = UserInfoDetail::default();
        let comment_list = (0..num)
            .map(|x| {
                let user_name = UserName::random().0;
                Comment {
                    id: x as i32,
                    address: Address::random().0,
                    userName: user_name.clone(),
                    likeNum: 10,
                    avatar: u.profileImage.clone(),
                    content: format!("{} said Hi", user_name.as_str()),
                }
            })
            .collect();
        getThoughtsCommentsBody {
            total: num,
            if_like: 0,
            comments: comment_list,
        }
    }
}

#[get("/getThoughtsComments?<thoughtId>")]
pub fn get_thought_comments(
    cookies: Cookies,
    conn: DbConn,
    thoughtId: i32,
) -> Json<HugResponse<Option<getThoughtsCommentsBody>>> {
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
    //get comments
    let res = comments::Comment::get_by_thought_id(&conn, thoughtId);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "get comment failed".to_string(),
            resultBody: None,
        });
    }
    let comment_db = res.unwrap();
    let cnt = comment_db.len();
    let mut body = getThoughtsCommentsBody::default(cnt as u32);
    let comment_list = body
        .comments
        .into_iter()
        .zip(comment_db)
        .map(|(mut x, y)| {
            x.id = y.id;
            x.address = y.address.clone();
            x.content = y.content.clone();
            let res = users::Users::get_user_by_address(&conn, y.address.clone());
            if res.is_ok() {
                if let Some(us) = res.unwrap().get(0) {
                    x.avatar = us.profile_image.clone();
                    x.userName = us.username.clone();
                }
            }
            x
        })
        .collect::<Vec<Comment>>();
    let mut if_like = 0;
    let res = likes::Likes::if_like(
        &conn,
        likes::NewLike {
            address: address,
            thought_id: thoughtId,
        },
    );
    if res {
        if_like = 1;
    } else {
        if_like = 0;
    }
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(getThoughtsCommentsBody {
            total: cnt as u32,
            if_like: if_like,
            comments: comment_list,
        }),
    })
}

#[derive(FromForm)]
pub struct ThoughtsCommentReq {
    pub thoughtId: i32,
    pub content: String,
}

#[post("/ThoughtsComment", data = "<thought_comment_req>")]
pub fn thoughts_comment(
    cookies: Cookies,
    conn: DbConn,
    thought_comment_req: Form<ThoughtsCommentReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    //check cookies
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let mut address = role.address.clone();

    let res = comments::Comment::create(
        &conn,
        comments::NewComment {
            thought_id: thought_comment_req.thoughtId,
            content: thought_comment_req.content.clone(),
            address: address,
        },
    );
    if res == false {
        return Json(HugResponse::new_failed("add comment failed", ""));
    }
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
pub struct delCommentReq {
    pub commentId: i32,
}

#[post("/delComment", data = "<del_comment_req>")]
pub fn del_comment(
    cookies: Cookies,
    conn: DbConn,
    del_comment_req: Form<delCommentReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    //check cookies
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let mut address = role.address.clone();

    let res = comments::Comment::delete(&conn, del_comment_req.commentId);
    if res == false {
        return Json(HugResponse::new_failed("delete comment failed", ""));
    }
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct LikeOrUnlikeCommentReq {
    pub commentId: i32,
    pub operate: i32,
}
#[post("/likeOrUnlikeComment", data = "<like_req>")]
pub fn like_or_unlike_comment(
    cookies: Cookies,
    conn: DbConn,
    like_req: LenientForm<LikeOrUnlikeCommentReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_success())
}
