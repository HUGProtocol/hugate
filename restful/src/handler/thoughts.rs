use super::{
    user::{Address, UserInfoDetail},
    *,
};

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct GetPopularThoughtsListReq {
    #[form(field = "type")]
    pub thought_type: String,
    pub address: String,
    pub currentPage: i32,
    pub pageSize: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Thought {
    pub address: String,
    pub userInfo: String,
    pub tips: String,
    pub content: String,
    pub likeNum: i32,
    pub pic: String,
    #[serde(rename = "type")]
    pub thought_type: String,
    pub avatar: String,
    pub pts: u64,
    pub sourceUrl: String,
}

impl Default for Thought {
    fn default() -> Self {
        let default_user = UserInfoDetail::default();
        Self {
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
) -> Json<HugResponse<GetPopularThoughtsListBody>> {
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: GetPopularThoughtsListBody::default(pop_thoughts_list_req.pageSize as u32),
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
    pub currentPage: i32,
    pub pageSize: i32,
}

#[post("/getMyThoughtsList", data = "<my_thoughts_list_req>")]
pub fn get_my_thoughts_list(
    conn: DbConn,
    my_thoughts_list_req: LenientForm<GetMyThoughtsListReq>,
) -> Json<HugResponse<GetPopularThoughtsListBody>> {
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: GetPopularThoughtsListBody::default(my_thoughts_list_req.pageSize as u32),
    })
}

#[derive(Serialize, Deserialize)]
pub struct ThoughtDetail {
    pub userName: String,
    pub tips: String,
    pub content: String,
    pub likeNum: i32,
    #[serde(rename = "type")]
    pub thought_type: String,
    pub avatar: String,
    pub pts: u64,
    pub sourceUrl: String,
    pub commentNum: u32,
    pub snapshot: String,
}

impl Default for ThoughtDetail {
    fn default() -> Self {
        let u = UserInfoDetail::default();
        let t = Thought::default();
        Self {
            userName: u.userName,
            tips: t.tips,
            content: t.content,
            likeNum: t.likeNum,
            thought_type: t.thought_type,
            avatar: u.profileImage,
            pts: t.pts,
            sourceUrl: t.sourceUrl,
            commentNum: 100,
            snapshot: t.pic,
        }
    }
}

#[get("/getThoughtDetail?<thoughtId>")]
pub fn get_thought_detail(db_conn: DbConn, thoughtId: i32) -> Json<HugResponse<ThoughtDetail>> {
    Json(HugResponse::new_success())
}

#[derive(FromForm)]
#[allow(non_snake_case)]
pub struct LikeOrUnlikeThoughtReq {
    pub thoughtId: i32,
    pub operate: i32,
}
#[post("/likeOrUnlikeThought", data = "<like_req>")]
pub fn like_or_unlike_thought(
    db_conn: DbConn,
    like_req: LenientForm<LikeOrUnlikeThoughtReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    Json(HugResponse::new_success())
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

