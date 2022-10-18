#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
use diesel::{prelude::*, table, Insertable, Queryable};
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

table! {
    flags (id) {
        id -> Int4,
        name -> Varchar,
        enabled -> Bool,
    }
}

#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[derive(Serialize, Deserialize, Clone, Queryable, Debug, Insertable)]
#[table_name = "flags"]
struct Flag {
    id: i32,
    name: String,
    enabled: bool,
}

#[get("/random")]
fn get_random_flag() -> Json<Flag> {
    Json(Flag {
        id: 1,
        name: "flag name".to_string(),
        enabled: true,
    })
}

#[get("/<id>")]
fn get_flag(id: i32) -> Json<Flag> {
    Json(Flag {
        id,
        name: "some name".to_string(),
        enabled: true,
    })
}

#[post("/", data = "<blog_post>")]
async fn create_flag(connection: Db, blog_post: Json<Flag>) -> Json<Flag> {
    connection
        .run(move |c| {
            diesel::insert_into(flags::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[get("/")]
async fn get_all_flags(connection: Db) -> Json<Vec<Flag>> {
    connection
        .run(|c| flags::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch flags")
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::fairing()).mount(
        "/flags",
        routes![
            get_random_flag,
            get_flag,
            get_all_flags,
            create_flag
        ],
    )
}
