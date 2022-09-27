use diesel::expression::ops::Add;
use rand::Rng;
use serde::__private::de;

use crate::{
    jwt::{check_cookies, verify_login_signature},
    models::users::{NewUser, Users},
};

// use crate::handler::*;
use super::*;

#[derive(Serialize, Deserialize, FromForm, Clone)]
#[allow(non_snake_case)]
pub struct CreateProfileReq {
    pub profileImage: String,
    pub name: String,
    pub email: String,
    pub twitter: String,
    pub about: String,
}

#[post("/createProfile", data = "<create_profile>")]
pub fn create_profile(
    cookies: Cookies,
    conn: DbConn,
    create_profile: LenientForm<CreateProfileReq>,
) -> Json<HugResponse<OneLineResultBody>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse::new_failed("check token failed", ""));
    }
    let role = res.unwrap();
    let address = role.address.clone();
    let res = Users::insert_or_update_user(
        NewUser {
            username: create_profile.name.clone(),
            email: create_profile.email.clone(),
            twitter: create_profile.twitter.clone(),
            about: create_profile.about.clone(),
            profile_image: create_profile.profileImage.clone(),
            address: address.clone(),
            pts: 0,
        },
        &conn,
    );
    if res {
        return Json(HugResponse::new_success());
    }
    Json(HugResponse::new_failed("insert into database failed", ""))
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
#[allow(non_snake_case)]
pub struct UserInfoAbstract {
    pub address: String,
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "profileImage")]
    pub profile_image: String,
}

impl UserInfoAbstract {
    pub fn random() -> Self {
        let detail = UserInfoDetail::random();
        return UserInfoAbstract {
            address: detail.address,
            username: detail.userName,
            profile_image: detail.profileImage,
        };
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Medal {
    pub id: i32,
    pub name: String,
    pub owner: String, //user address
    pub image: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct UserInfoDetail {
    pub address: String,
    pub userName: String,
    pub profileImage: String,
    pub followersNum: i32,
    pub followingNum: i32,
    pub followersList: Vec<UserInfoAbstract>,
    pub pts: i64,
    pub medalNum: i32,
    pub medalList: Vec<Medal>,
    pub email: String,
    pub twitter: String,
    pub about: String,
}

pub struct Address(pub String);
impl Default for Address {
    fn default() -> Self {
        Address("0x9C3739D43a89cedf167204550267797F5931ebF5".to_string())
    }
}
impl Address {
    pub fn random() -> Self {
        const CHARSET: &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm1234567890";
        let mut rng = rand::thread_rng();
        let address: String = (0..40)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        Address(address)
    }
}

#[derive(Clone)]
pub struct UserName(pub String);
impl Default for UserName {
    fn default() -> Self {
        UserName("CaleHH".to_string())
    }
}
impl UserName {
    pub fn random() -> Self {
        const CHARSET: &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm1234567890";
        let mut rng = rand::thread_rng();
        let username: String = (0..6)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        UserName(username)
    }
}

pub struct MedalList(Vec<Medal>);
impl Default for MedalList {
    fn default() -> Self {
        MedalList(vec![
            Medal {
                id: 1,
                name: "Punk".to_string(),
                owner: Address::random().0,
                image: "https://lh3.googleusercontent.com/fWqZfbuMuVvba6yxqAxl3dyxMNPvHiHXK2bqovdf7WnTD9tPNryphX7REIWFSlLam1x3gc7XJO0K1rkiSzn5_RcS=w600".to_string(),
            },
            Medal {
                id: 2,
                name: "Geek".into(),
                owner: Address::random().0,
                image: "https://lh3.googleusercontent.com/X2IBRGGUe6RBZgSGUl0umI2vDjxW7ionN7wdM_cy8mM5JCtzLUWfUiXxiYUkNQg4l-EMofuh-qrgO38gc36GSjpLeUp4ul7y9MRq=w600".to_string(),
            },
            Medal {
                id: 3,
                name: "Winner".to_string(),
                owner: Address::random().0,
                image: "https://lh3.googleusercontent.com/VPMI-SF8f-FX99wNP3rSmvGdSC9xcXHjUk-W2UgjB6smj-xf_KXdgUuZgP4QA6JvPmg8lYMQ0KCr4cWGSvaSKiK8gFVf8D4tJqwKdg=w600".to_string(),
            },
        ])
    }
}

impl UserInfoDetail {
    pub fn random() -> Self {
        Self {
            address: Address::random().0,
            userName: UserName::random().0,
            profileImage: "https://img.seadn.io/files/0507ede2bd1c13e5b2c99fa98ac3b085.png"
                .to_string(),
            followersNum: 10,
            followingNum: 100,
            followersList: Default::default(),
            pts: 1000,
            medalNum: 3,
            medalList: MedalList::default().0,
            email: format!("{}@gmail.com", UserName::random().0),
            twitter: UserName::random().0,
            about: "fly me to the moon".to_string(),
        }
    }
}

impl Default for UserInfoDetail {
    fn default() -> Self {
        Self {
            address: Address::default().0,
            userName: UserName::default().0,
            profileImage: "https://img.seadn.io/files/0507ede2bd1c13e5b2c99fa98ac3b085.png"
                .to_string(),
            followersNum: 10,
            followingNum: 10,
            followersList: Default::default(),
            pts: 1000,
            medalNum: 3,
            medalList: MedalList::default().0,
            email: "calehh@gmail.com".to_string(),
            twitter: "William".to_string(),
            about: "fly me to the moon".to_string(),
        }
    }
}

#[get("/getUserInfo?<address>")]
pub fn get_user_info(
    cookies: Cookies,
    conn: DbConn,
    address: Option<String>,
) -> Json<HugResponse<Option<UserInfoDetail>>> {
    let res = check_cookies(&cookies);
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "check token failed".to_string(),
            resultBody: None,
        });
    }
    let role = res.unwrap();
    let address = address.unwrap_or(role.address);
    // let address = role.address.clone();
    let res = Users::get_user_by_address(&conn, address.clone());
    if res.is_err() {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "database connection failed".to_string(),
            resultBody: None,
        });
    };
    let user_vec = res.unwrap();
    if user_vec.len() == 0 {
        return Json(HugResponse {
            resultCode: 500,
            resultMsg: "user not exist".to_string(),
            resultBody: None,
        });
    }
    let mut user_info = UserInfoDetail::default();
    let user_select = user_vec.get(0).unwrap();
    user_info.address = user_select.address.clone();
    user_info.userName = user_select.username.clone();
    user_info.profileImage = user_select.profile_image.clone();
    user_info.pts = user_select.pts.clone();
    user_info.email = user_select.email.clone();
    user_info.twitter = user_select.twitter.clone();
    user_info.about = user_select.about.clone();

    //get followers
    let res = Users::get_user_by_followee(&conn, address.clone());
    if res.is_ok() {
        let followers = res.unwrap();
        user_info.followersNum = followers.len() as i32;
        user_info.followersList = followers;
    }

    // get following count
    let res = Users::get_following_count(&conn, address.clone());
    if res.is_ok() {
        user_info.followingNum = res.unwrap() as i32;
    }

    return Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: Some(user_info),
    });
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginReq {
    pub address: String,
    pub timestamp: i32,
    pub sigType: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRes {
    pub JWT: String,
}

#[post("/login", data = "<login_req>")]
pub fn login(mut cookies: Cookies, login_req: Form<LoginReq>) -> Json<HugResponse<LoginRes>> {
    //todo: verify signature
    // let res = verify_login_signature(
    //     login_req.address.clone(),
    //     login_req.timestamp as u64,
    //     login_req.signature.clone(),
    // );
    // if res.is_err() {
    //     return Json(HugResponse::new_failed(
    //         "verify signature failed",
    //         res.err().unwrap().to_string().as_str(),
    //     ));
    // }
    let jwt = crate::jwt::jwt_generate(login_req.address.clone(), login_req.timestamp as u64);
    cookies.add(Cookie::new("jwt".to_string(), jwt.clone()));
    Json(HugResponse {
        resultCode: 200,
        resultMsg: "success".to_string(),
        resultBody: LoginRes { JWT: jwt },
    })
}
