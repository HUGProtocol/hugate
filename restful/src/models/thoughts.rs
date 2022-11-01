use super::pagination::Paginate;
use crate::handler::user::UserInfoAbstract;
use crate::schema::follow::dsl::follow as all_follows;
use crate::schema::thoughts::dsl::thoughts as all_thoughts;
use crate::schema::thoughts::likes;
use crate::schema::users::dsl::users as all_users;
use crate::schema::{self, follow, likes as likes_table, thoughts, users};
use chrono::NaiveDateTime;
use diesel::dsl::not;
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
    pub pts: i64,
    pub embeded: String,
    pub html_backup: String,
    pub token_id: i64,
}

#[derive(Debug, Insertable, AsChangeset)]
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
    pub embeded: String,
    pub html_backup: String,
    pub token_id: i64,
}

impl Thoughts {
    pub fn get_popular(
        conn: &PgConnection,
        page: i64,
        per_page: i64,
        thought_type: Option<String>,
        address: Option<String>,
        submit_state: Option<String>,
        order_by: Option<i32>,
    ) -> Result<(Vec<Thoughts>, i64), Error> {
        // let mut query = all_thoughts.order(thoughts::likes.desc()).into_boxed();
        let mut query = all_thoughts.into_boxed();
        query = match order_by {
            Some(1) => query.order_by(thoughts::id.desc()),
            _ => query.order_by(thoughts::likes.desc()),
        };
        if let Some(thought_type) = thought_type {
            query = query.filter(thoughts::thought_type.eq(thought_type));
        }
        if let Some(address) = address {
            query = query.filter(thoughts::thought_type.eq(address));
        }
        if let Some(submit_state) = submit_state {
            query = query.filter(thoughts::submit_state.eq(submit_state));
        }
        query = query.filter(not(thoughts::viewed.eq("self")));
        query = query.filter(not(thoughts::submit_state.eq("save")));
        let query_page = query.paginate(page).per_page(per_page);
        query_page.load_and_count_pages(conn)
    }

    pub fn get_my_like(
        conn: &PgConnection,
        address: String,
        page: i64,
        per_page: i64,
        thought_type: Option<String>,
        viewed: Option<String>,
    ) -> Result<(Vec<Thoughts>, i64), Error> {
        // let filtered = likes_table::dsl::likes.filter(likes_table::address.eq(address));
        let joined = thoughts::table
            .left_join(likes_table::table.on(likes_table::thought_id.eq(thoughts::id)))
            .filter(likes_table::address.eq(address));
        let mut query = joined
            .order(thoughts::id.desc())
            .select((
                thoughts::id,
                thoughts::content,
                thoughts::address,
                thoughts::tips,
                thoughts::thought_type,
                thoughts::source_url,
                thoughts::snapshot,
                thoughts::create_at,
                thoughts::updated_at,
                thoughts::likes,
                thoughts::viewed,
                thoughts::submit_state,
                thoughts::html,
                thoughts::pts,
                thoughts::embeded,
                thoughts::html_backup,
                thoughts::token_id,
            ))
            .into_boxed();

        if let Some(viewed) = viewed {
            query = query.filter(thoughts::viewed.eq(viewed));
        }
        if let Some(thought_type) = thought_type {
            query = query.filter(thoughts::thought_type.eq(thought_type));
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
        state: Option<String>,
    ) -> Result<(Vec<Thoughts>, i64), Error> {
        let mut query = all_thoughts.order(thoughts::id.desc()).into_boxed();
        if let Some(viewed) = viewed {
            query = query.filter(thoughts::viewed.eq(viewed));
        }
        if let Some(thought_type) = thought_type {
            query = query.filter(thoughts::thought_type.eq(thought_type));
        }
        if let Some(submit_state) = state {
            query = query.filter(thoughts::submit_state.eq(submit_state));
        }
        query = query.filter(thoughts::address.eq(address));
        let query_page = query.paginate(page).per_page(per_page);
        query_page.load_and_count_pages(conn)
    }

    pub fn get_by_id(conn: &PgConnection, id: i32) -> Result<Vec<Thoughts>, Error> {
        all_thoughts
            .filter(thoughts::id.eq(id))
            .distinct()
            .load(conn)
    }

    pub fn create(conn: &PgConnection, new_thought: NewThought) -> Result<i32, Error> {
        let res: Vec<Thoughts> = diesel::insert_into(thoughts::table)
            .values(&new_thought)
            .get_results(conn)?;
        Ok(res.get(0).ok_or(Error::NotFound)?.id)
    }

    pub fn update(conn: &PgConnection, new_thought: NewThought, thought_id: i32) -> bool {
        diesel::update(thoughts::dsl::thoughts)
            .filter(thoughts::id.eq(thought_id))
            .set(&new_thought)
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

    pub fn update_pts(conn: &PgConnection, thought_id: i32, pts: i64) -> bool {
        diesel::update(thoughts::dsl::thoughts)
            .filter(thoughts::id.eq(thought_id))
            .set(thoughts::pts.eq(pts))
            .execute(conn)
            .is_ok()
    }
}
