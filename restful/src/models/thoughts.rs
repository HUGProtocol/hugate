use super::pagination::Paginate;
use crate::handler::user::UserInfoAbstract;
use crate::schema::follow::dsl::follow as all_follows;
use crate::schema::thoughts::dsl::thoughts as all_thoughts;
use crate::schema::thoughts::likes;
use crate::schema::users::dsl::users as all_users;
use crate::schema::{self, follow, thoughts, users};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Debug, Queryable)]
pub struct Thoughts {
    pub id: i32,
    pub content: String,
    pub address: String,
    pub tips: String,
    pub thought_type: String,
    pub source_url: String,
    pub snapshot: String,
    pub create_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub likes: i64,
    pub viewed: String,
    pub submit_state: String,
    pub html: String,
}

#[derive(Debug, Insertable)]
#[table_name = "thoughts"]
pub struct NewThought {
    pub content: String,
    pub address: String,
    pub tips: String,
    pub thought_type: String,
    pub source_url: String,
    pub snapshot: String,
    pub viewed: String,
    pub submit_state: String,
    pub html: String,
}

impl Thoughts {
    pub fn get_popular(
        conn: &PgConnection,
        page: i64,
        per_page: i64,
        thought_type: Option<String>,
        address: Option<String>,
        submit_state: Option<String>,
    ) -> Result<(Vec<Thoughts>, i64), Error> {
        let query = all_thoughts.order(thoughts::likes.desc());
        if let Some(thought_type) = thought_type {
            query.filter(thoughts::thought_type.eq(thought_type));
        }
        if let Some(address) = address {
            query.filter(thoughts::thought_type.eq(address));
        }
        if let Some(submit_state) = submit_state {
            query.filter(thoughts::submit_state.eq(submit_state));
        }
        let query_page = query.paginate(page).per_page(per_page);
        query_page.load_and_count_pages(conn)
    }

    pub fn get_my(
        conn: &PgConnection,
        address: String,
        page: i64,
        per_page: i64,
        thought_type: Option<String>,
        viewed: Option<String>,
    ) -> Result<(Vec<Thoughts>, i64), Error> {
        let query = all_thoughts.order(thoughts::id.desc());
        if let Some(viewed) = viewed {
            query.filter(thoughts::viewed.eq(viewed));
        }
        if let Some(thought_type) = thought_type {
            query.filter(thoughts::thought_type.eq(thought_type));
        }
        query.filter(thoughts::address.eq(address));
        let query_page = query.paginate(page).per_page(per_page);
        query_page.load_and_count_pages(conn)
    }

    pub fn get_by_id(conn: &PgConnection, id: i32) -> Result<Vec<Thoughts>, Error> {
        all_thoughts
            .filter(thoughts::id.eq(id))
            .distinct()
            .load(conn)
    }

    pub fn create(conn: &PgConnection, new_thought: NewThought) -> bool {
        diesel::insert_into(thoughts::table)
            .values(&new_thought)
            .execute(conn)
            .is_ok()
    }

    pub fn add_like(conn: &PgConnection, thought_id: i32) -> bool {
        diesel::update(thoughts::dsl::thoughts)
            .filter(thoughts::id.eq(thought_id))
            .set(thoughts::likes.eq(likes + 1))
            .execute(conn)
            .is_ok()
    }
    pub fn reduce_unlike(conn: &PgConnection, thought_id: i32) -> bool {
        diesel::update(thoughts::dsl::thoughts)
            .filter(thoughts::id.eq(thought_id))
            .set(thoughts::likes.eq(likes - 1))
            .execute(conn)
            .is_ok()
    }
}
