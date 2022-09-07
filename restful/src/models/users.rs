use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

// this is to get users from the database
#[derive(Serialize, Queryable)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}