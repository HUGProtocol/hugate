#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(
    proc_macro_derive_resolution_fallback,
    unused_attributes,
    non_snake_case,
    dead_code
)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use handler::comments::{
    static_rocket_route_info_for_del_comment, static_rocket_route_info_for_get_thought_comments,
    static_rocket_route_info_for_thoughts_comment,
};
use handler::*;
use std::env;
use std::process::Command;

mod db;
mod handler;
mod jwt;
mod models;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    println!("url : {}",database_url);
    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/hug",
            routes![
                user::create_profile,
                user::get_user_info,
                user::login,
                comments::get_thought_comments,
                comments::thoughts_comment,
                comments::del_comment,
                follow::follow_or_not,
                follow::get_follow_list,
                medal::get_medal_list,
                thoughts::get_popular_thoughts_list,
                thoughts::get_my_thoughts_list,
                thoughts::get_thought_detail,
                thoughts::like_or_unlike_thought,
                thoughts::reward,
                thoughts::createThoughts,
            ],
        )
        .register(catchers![not_found, miss_variable])
}

fn main() {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cd ui && npm start"])
            .spawn()
            .expect("Failed to start UI Application")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cd ui && npm start")
            .spawn()
            .expect("Failed to start UI Application")
    };
    rocket().launch();
}
