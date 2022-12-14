use crate::handler::user::UserInfoAbstract;
use crate::schema::follow::dsl::follow as all_follows;
use crate::schema::users::dsl::users as all_users;
use crate::schema::{self, follow, users};
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use super::ReplaceIPFSUrl;

#[derive(Debug, Queryable, Serialize, Default, Clone)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub profile_image: String,
    pub email: String,
    pub twitter: String,
    pub about: String,
    pub pts: i64,
    #[serde(skip_serializing)]
    pub create_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
    pub address: String,
    pub banner: String,
}

#[derive(Debug, Queryable, Insertable, Default, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub profile_image: String,
    pub email: String,
    pub twitter: String,
    pub about: String,
    pub pts: i64,
    pub address: String,
    pub banner: String,
}

#[derive(Debug, Queryable, Insertable, Default)]
#[table_name = "follow"]
pub struct NewFollow {
    pub follower: String,
    pub followee: String,
}

impl Users {
    pub fn get_all_users(conn: &PgConnection) -> Result<Vec<Users>, Error> {
        let mut res: Result<Vec<Users>, Error> =
            all_users.order(users::id.desc()).load::<Users>(conn);
        if let Ok(ref mut res) = res {
            for t in res {
                t.profile_image = ReplaceIPFSUrl(t.profile_image.clone());
                t.banner = ReplaceIPFSUrl(t.banner.clone());
            }
        }
        res
    }

    pub fn insert_or_update_user(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict(users::address)
            .do_update()
            .set(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_address(conn: &PgConnection, addr: String) -> Result<Vec<Users>, Error> {
        let mut res: Result<Vec<Users>, Error> = all_users
            .filter(schema::users::dsl::address.eq(addr))
            .distinct()
            .load::<Users>(conn);
        if let Ok(ref mut res) = res {
            for t in res {
                t.profile_image = ReplaceIPFSUrl(t.profile_image.clone());
                t.banner = ReplaceIPFSUrl(t.banner.clone());
            }
        }
        res
    }

    pub fn get_user_by_followee(
        conn: &PgConnection,
        addr: String,
    ) -> Result<Vec<UserInfoAbstract>, Error> {
        let joined = users::table.left_join(follow::table.on(follow::follower.eq(users::address)));
        let mut res: Result<Vec<UserInfoAbstract>, Error> = joined
            .filter(follow::followee.eq(addr))
            .select((users::address, users::username, users::profile_image))
            .load::<UserInfoAbstract>(conn);
        if let Ok(ref mut res) = res {
            for t in res {
                t.profile_image = ReplaceIPFSUrl(t.profile_image.clone());
            }
        }
        res
    }

    pub fn follow(new_follow: NewFollow, conn: &PgConnection) -> bool {
        diesel::insert_into(follow::table)
            .values(&new_follow)
            .execute(conn)
            .is_ok()
    }

    pub fn unfollow(old_follow: NewFollow, conn: &PgConnection) -> bool {
        diesel::delete(
            all_follows
                .filter(follow::follower.eq(old_follow.follower))
                .filter(follow::followee.eq(old_follow.followee)),
        )
        .execute(conn)
        .is_ok()
    }

    pub fn if_follow(follower: String, followee: String, conn: &PgConnection) -> bool {
        let res: Result<i64, Error> = all_follows
            .filter(follow::followee.eq(followee.clone()))
            .filter(follow::follower.eq(follower.clone()))
            .select(diesel::dsl::count(follow::dsl::id))
            .first(conn);
        if let Ok(cnt) = res {
            println!(
                "follower {} followee {} cnt {}",
                follower.as_str(),
                followee.as_str(),
                cnt
            );
            if cnt > 0 {
                return true;
            }
        }
        return false;
        // if let Ok(cnt) = all_follows
        //     .filter(follow::followee.eq(followee.clone()))
        //     .filter(follow::follower.eq(follower.clone()))
        //     .count()
        //     .get_result(conn)
        // {
        //     println!(
        //         "follower {} followee {} cnt {}",
        //         follower.as_str(),
        //         followee.as_str(),
        //         cnt
        //     );
        //     if cnt > 0 {
        //         return true;
        //     }
        // }
        // return false;
    }

    //followers
    pub fn get_followers(
        conn: &PgConnection,
        addr: String,
    ) -> Result<Vec<UserInfoAbstract>, Error> {
        let joined = users::table.left_join(follow::table.on(follow::follower.eq(users::address)));
        let mut res: Result<Vec<UserInfoAbstract>, Error> = joined
            .filter(follow::followee.eq(addr))
            .select((users::address, users::username, users::profile_image))
            .load::<UserInfoAbstract>(conn);
        if let Ok(ref mut res) = res {
            for t in res {
                t.profile_image = ReplaceIPFSUrl(t.profile_image.clone());
            }
        }
        res
    }

    //followees
    pub fn get_followees(
        conn: &PgConnection,
        addr: String,
    ) -> Result<Vec<UserInfoAbstract>, Error> {
        let joined = users::table.left_join(follow::table.on(follow::followee.eq(users::address)));
        let mut res: Result<Vec<UserInfoAbstract>, Error> = joined
            .filter(follow::follower.eq(addr))
            .select((users::address, users::username, users::profile_image))
            .load::<UserInfoAbstract>(conn);
        if let Ok(ref mut res) = res {
            for t in res {
                t.profile_image = ReplaceIPFSUrl(t.profile_image.clone());
            }
        }
        res
    }

    pub fn get_following_count(conn: &PgConnection, addr: String) -> Result<i64, Error> {
        follow::dsl::follow
            .filter(follow::dsl::follower.eq(addr))
            .count()
            .get_result(conn)
    }

    pub fn update_pts(addr: String, conn: &PgConnection, pts: i64) -> bool {
        diesel::update(schema::users::dsl::users)
            .filter(schema::users::dsl::address.eq(addr))
            .set(schema::users::dsl::pts.eq(pts))
            .execute(conn)
            .is_ok()
    }

    pub fn add_pts(addr: String, conn: &PgConnection, pts: i64) -> bool {
        diesel::update(schema::users::dsl::users)
            .filter(schema::users::dsl::address.eq(addr))
            .set(schema::users::dsl::pts.eq(schema::users::dsl::pts + pts))
            .execute(conn)
            .is_ok()
    }

    pub fn reduce_pts(addr: String, conn: &PgConnection, pts: i64) -> bool {
        diesel::update(schema::users::dsl::users)
            .filter(schema::users::dsl::address.eq(addr))
            .set(schema::users::dsl::pts.eq(schema::users::dsl::pts - pts))
            .execute(conn)
            .is_ok()
    }
}
