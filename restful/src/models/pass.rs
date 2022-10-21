use super::pagination::Paginate;
use crate::schema::pass::dsl::pass as all_pass;
use crate::schema::thoughts::dsl::thoughts as all_thoughts;
use crate::schema::users::dsl::users as all_users;
use crate::schema::{self, follow, pass, thoughts, users};
use chrono::NaiveDateTime;
use diesel::dsl::not;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use web3::types::Res;

#[derive(Debug, Queryable)]
pub struct Pass {
    pub id: i32,
    pub thought_id: i64,
    pub token_id: i64,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "pass"]
pub struct NewPass {
    pub thought_id: i64,
    pub token_id: i64,
}

impl Pass {
    pub fn get_by_thought(conn: &PgConnection, thought_id: i64) -> Result<Vec<Pass>, Error> {
        all_pass.filter(pass::thought_id.eq(thought_id)).load(conn)
    }
    pub fn get_by_token(conn: &PgConnection, token_id: i64) -> Result<Vec<Pass>, Error> {
        all_pass.filter(pass::token_id.eq(token_id)).load(conn)
    }
    pub fn put_pass(conn: &PgConnection, new_pass: NewPass) -> bool {
        diesel::insert_into(pass::table)
            .values(&new_pass)
            .on_conflict(pass::thought_id)
            .do_update()
            .set(pass::token_id.eq(new_pass.token_id))
            .execute(conn)
            .is_ok()
    }
}
