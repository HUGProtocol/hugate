use super::pagination::Paginate;
use crate::handler::user::UserInfoAbstract;
use crate::schema::comment::dsl::comment as all_comments;
use crate::schema::follow::dsl::follow as all_follows;
use crate::schema::likes::dsl::likes as all_likes;
use crate::schema::thoughts::dsl::thoughts as all_thoughts;
use crate::schema::users::dsl::users as all_users;
use crate::schema::{self, comment, follow, likes, thoughts, users};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Debug, Queryable)]
pub struct Comment {
    pub id: i32,
    pub thought_id: i32,
    pub address: String,
    pub content: String,
    pub create_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "comment"]
pub struct NewComment {
    pub thought_id: i32,
    pub address: String,
    pub content: String,
}

impl Comment {
    pub fn get_by_thought_id(conn: &PgConnection, thought_id: i32) -> Result<Vec<Comment>, Error> {
        all_comments
            .filter(comment::thought_id.eq(thought_id))
            .load(conn)
    }

    pub fn get_count_by_thought_id(conn: &PgConnection, thought_id: i32) -> Result<i64, Error> {
        all_comments
            .filter(comment::thought_id.eq(thought_id))
            .count()
            .get_result(conn)
    }

    pub fn create(conn: &PgConnection, new_comment: NewComment) -> bool {
        diesel::insert_into(comment::table)
            .values(&new_comment)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(conn: &PgConnection, comment_id: i32) -> bool {
        diesel::delete(comment::table.filter(comment::id.eq(comment_id)))
            .execute(conn)
            .is_ok()
    }
}
