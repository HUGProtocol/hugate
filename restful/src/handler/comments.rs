use super::{
    user::{Address, UserInfoDetail, UserName},
    *,
};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub address: String,
    pub userName: String,
    pub likeNum: u32,
    pub avatar: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct getThoughtsCommentsBody {
    pub total: u32,
    pub comments: Vec<Comment>,
}

impl getThoughtsCommentsBody {
    pub fn default(num: u32) -> Self {
        let u = UserInfoDetail::default();
        let comment_list = (0..num)
            .map(|x| {
                let user_name = UserName::random().0;
                Comment {
                    id: format!("{}", x),
                    address: Address::random().0,
                    userName: user_name.clone(),
                    likeNum: x + 100,
                    avatar: u.profileImage.clone(),
                    content: format!("{} said Hi", user_name.as_str()),
                }
            })
            .collect();
        getThoughtsCommentsBody {
            total: num,
            comments: comment_list,
        }
    }
}

#[get("/getThoughtsComments?<thoughtId>")]
pub fn get_thought_comments(
    db_conn: DbConn,
    thoughtId: i32,
) -> Json<HugResponse<getThoughtsCommentsBody>> {
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: getThoughtsCommentsBody::default(5),
    })
}

#[derive(FromForm)]
pub struct ThoughtsCommentReq {
    pub thoughtId: i32,
    pub content: String,
}

#[post("/ThoughtsComment", data = "<thought_comment_req>")]
pub fn thoughts_comment(
    db_conn: DbConn,
    thought_comment_req: Form<ThoughtsCommentReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
pub struct delCommentReq {
    pub commentId: i32,
}

#[post("/delComment", data = "<del_comment_req>")]
pub fn del_comment(
    db_conn: DbConn,
    del_comment_req: Form<delCommentReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_success())
}