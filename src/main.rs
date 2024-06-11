extern crate diesel;

mod models;
pub mod schema;

use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("db url variable no encontrada");
    let mut conn: PgConnection =
        PgConnection::establish(&db_url).expect("error conectándose a la base de datos");

    use crate::models::{NewPost, Post};
    use crate::schema::posts::dsl::*;

    let new_post = NewPost{
        title: "Mi primer blogpost".to_string(),
        body: "Aqui esta mi descripcion".to_string(),
        slug: "primer-post".to_string(),
    };

    diesel::insert_into(posts::table())
        .values(&new_post)
        .get_result::<Post>(&mut conn)
        .expect("error insertando primer registro");

    let posts_result: Vec<Post> = posts
        .load::<Post>(&mut conn)
        .expect("error al ejecutar la Query posts load");

    for post in posts_result {
        println!("{}", post.title);
    }
}
