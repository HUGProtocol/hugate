use diesel::PgConnection;

use super::init_pool;
use crate::models::users::{NewFollow, NewUser, Users};

#[test]
fn insert_users() {
    let db_url = "".to_string();
    // let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(db_url);
    let pool = init_pool(db_url);
    //     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    //    let poool =  pool.clone();
    let conn = pool.get().unwrap();
    let mut new_user = NewUser::default();
    new_user.username = "haa".into();
    new_user.address = "0x9C3739D43a89cedf167204550267797F5931ebF1".into();
    let res = Users::insert_user(new_user, &conn);
    assert_eq!(res, true);
}

#[test]
fn follow() {
    let db_url = "".to_string();
    // let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(db_url);
    let pool = init_pool(db_url);
    //     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    //    let poool =  pool.clone();
    let conn = pool.get().unwrap();
    let new_follow = NewFollow {
        follower: "0x9C3739D43a89cedf167204550267797F5931ebF1".into(),
        followee: "0xBb57EdbAaB0F56ECF494e77a73D5Fd951C295d48".into(),
    };
    let res = Users::follow(new_follow, &conn);
    assert_eq!(res, true);
}

#[test]
fn get_followers() {
    let db_url = "".to_string();
    // let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(db_url);
    let pool = init_pool(db_url);
    //     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    //    let poool =  pool.clone();
    let conn = pool.get().unwrap();
    let user_vec =
        Users::get_user_by_followee(&conn, "0xBb57EdbAaB0F56ECF494e77a73D5Fd951C295d48".into());
    println!("{:?}", user_vec);
}

#[test]
fn get_users() {
    let db_url = "".to_string();
    // let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(db_url);
    let pool = init_pool(db_url);
    //     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    //    let poool =  pool.clone();
    let conn = pool.get().unwrap();
    let user_vec = Users::get_all_users(&conn).unwrap();
    println!("{:?}", user_vec);
}

#[test]
fn get_users_by_address() {
    let db_url = "".to_string();
    // let manager = r2d2_diesel::ConnectionManager::<PgConnection>::new(db_url);
    let pool = init_pool(db_url);
    //     let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    //    let poool =  pool.clone();
    let conn = pool.get().unwrap();
    let user_vec =
        Users::get_user_by_address(&conn, "0x9C3739D43a89cedf167204550267797F5931ebF5".into())
            .unwrap();
    println!("{:?}", user_vec);
}
