use crate::schema::metadata::dsl::metadata as all_metadata;
use crate::schema::pass::dsl::pass as all_pass;
use crate::schema::{metadata, pass};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

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

#[derive(Debug, Queryable, Serialize, Clone, FromForm)]
pub struct Metadata {
    pub id: i32,
    pub meta_json: String,
    pub address: String,
    pub token_id: i64,
    pub nft_type: String,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Clone)]
#[table_name = "metadata"]
pub struct NewMetadata {
    pub meta_json: String,
    pub address: String,
    pub token_id: i64,
    pub nft_type: String,
}

impl Metadata {
    pub fn new(conn: &PgConnection, new_metadata: &NewMetadata) -> bool {
        diesel::insert_into(metadata::table)
            .values(new_metadata)
            .execute(conn)
            .is_ok()
    }

    pub fn update(conn: &PgConnection, new_metadata: &NewMetadata, id: i32) -> bool {
        diesel::update(metadata::dsl::metadata)
            .filter(metadata::id.eq(id))
            .filter(metadata::address.eq(new_metadata.address.clone()))
            .set(new_metadata)
            .execute(conn)
            .is_ok()
    }

    pub fn get_by_id(conn: &PgConnection, id: i32) -> Result<Vec<Metadata>, Error> {
        all_metadata
            .filter(metadata::id.eq(id))
            .distinct()
            .load(conn)
    }

    pub fn get_by_token_id(conn: &PgConnection, token_id: i64) -> Result<Vec<Metadata>, Error> {
        all_metadata
            .filter(metadata::token_id.eq(token_id))
            .distinct()
            .load(conn)
    }

    pub fn get_by_address(conn: &PgConnection, address: String) -> Result<Vec<Metadata>, Error> {
        all_metadata
            .filter(metadata::address.eq(address))
            .distinct()
            .load(conn)
    }

    pub fn get_by_token_id_vec(
        conn: &PgConnection,
        token_id_vec: Vec<i32>,
    ) -> Result<Vec<Metadata>, Error> {
        let mut query = all_metadata.into_boxed();
        let mut cnt = 0;
        for token_id in token_id_vec {
            cnt += 1;
            if cnt > 10 {
                break;
            }
            query = query.or_filter(metadata::token_id.eq(token_id as i64));
        }
        query.get_results(conn)
    }
}
