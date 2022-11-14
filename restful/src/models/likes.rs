use crate::schema::likes;
use crate::schema::likes::dsl::likes as all_likes;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Debug, Queryable)]
pub struct Likes {
    pub id: i32,
    pub thought_id: i32,
    pub address: String,
    pub create_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "likes"]
pub struct NewLike {
    pub thought_id: i32,
    pub address: String,
}

impl Likes {
    pub fn like(conn: &PgConnection, new_like: NewLike) -> bool {
        diesel::insert_into(likes::table)
            .values(&new_like)
            .execute(conn)
            .is_ok()
    }

    pub fn unlike(conn: &PgConnection, new_unlike: NewLike) -> bool {
        diesel::delete(
            all_likes
                .filter(likes::address.eq(new_unlike.address.clone()))
                .filter(likes::thought_id.eq(new_unlike.thought_id)),
        )
        .execute(conn)
        .is_ok()
    }

    pub fn if_like(conn: &PgConnection, new_unlike: NewLike) -> bool {
        let res: Result<i64, Error> = all_likes
            .filter(likes::address.eq(new_unlike.address))
            .filter(likes::thought_id.eq(new_unlike.thought_id))
            .count()
            .get_result(conn);

        if res.is_err() {
            return false;
        }

        if res.unwrap() <= 0 {
            return false;
        }
        true
    }

    pub fn count(conn: &PgConnection, thought_id: i32) -> Result<i64, Error> {
        all_likes
            .filter(likes::thought_id.eq(thought_id))
            .count()
            .get_result(conn)
    }
}
