#![allow(non_camel_case_types)]
use super::{
    user::{Address, UserName},
    *,
};

#[derive(Serialize, Deserialize)]
pub struct medal {
    pub nftAddress: String,
    pub nftName: String,
    pub nftIco: String,
    pub nftNum: i32,
}

#[derive(Serialize, Deserialize)]
pub struct medalListBody {
    pub nfts: Vec<medal>,
    pub total: u32,
}

impl medalListBody {
    fn default(num: u32) -> Self {
        let medal_list = (0..num).map(|_|{
            medal{ nftAddress: Address::random().0, nftName: UserName::random().0 , nftNum: 100, nftIco: "https://lh3.googleusercontent.com/fWqZfbuMuVvba6yxqAxl3dyxMNPvHiHXK2bqovdf7WnTD9tPNryphX7REIWFSlLam1x3gc7XJO0K1rkiSzn5_RcS=w600".to_string() }
        }).collect::<Vec<medal>>();
        Self {
            nfts: medal_list,
            total: num,
        }
    }
}

#[get("/getMedalList?<_currentPage>&<pageSize>")]
pub fn get_medal_list(
    _db_conn: DbConn,
    _currentPage: Option<i32>,
    pageSize: Option<i32>,
) -> Json<HugResponse<medalListBody>> {
    let num = pageSize.unwrap_or(5);
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: medalListBody::default(num as u32),
    })
}
