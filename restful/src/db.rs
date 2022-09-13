use diesel::pg::PgConnection;

use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// use crate::schema::users;
// use crate::schema::users::dsl::users as all_users;
// use chrono::NaiveDateTime;
// use diesel::prelude::*;
// #[derive(Debug, Queryable)]
// pub struct Users {
//     pub id: i32,
//     pub username: String,
//     pub profile_image: String,
//     pub email: String,
//     pub twitter: String,
//     pub about: String,
//     pub pts: i64,
//     pub create_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub address: String,
// }

// #[derive(Debug, Queryable, Insertable, Default)]
// #[table_name = "users"]
// pub struct NewUser {
//     pub username: String,
//     pub profile_image: String,
//     pub email: String,
//     pub twitter: String,
//     pub about: String,
//     pub pts: i64,
//     pub address: String,
// }

// impl Users {
//     pub fn get_all_users(conn: &PgConnection) -> Vec<Users> {
//         all_users
//             .order(users::id.desc())
//             .load::<Users>(conn)
//             .expect("error!")
//     }

//     pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
//         diesel::insert_into(users::table)
//             .values(&user)
//             .execute(conn)
//             .is_ok()
//     }
// }

#[cfg(test)]
mod tests;
