extern crate diesel;

mod models {
    pub mod post;
}
mod manager {
    pub mod post;
}
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let db_url: String = env::var("DATABASE_URL").expect("db url variable no encontrada");
    let mut conn: PgConnection =
        PgConnection::establish(&db_url).expect("error conectándose a la base de datos");

    use crate::manager::post::PostsManager;
    use crate::models::post::{NewPost, Post};

    let create_result: Post = PostsManager::create_new_post(
        &mut conn,
        NewPost {
            title: "Mi tercer blogpost".to_string(),
            body: "Aqui esta mi descripcion".to_string(),
            slug: "tercer-post".to_string(),
        },
    );

    println!("se creo exitosamente el registro: {}", create_result.title);

    let posts_result: Vec<Post> = PostsManager::get_all_posts(&mut conn);

    for post in posts_result {
        println!("se creo el post con titulo: {}", post.title)
    }

    let posts_filter_by_slug_result: Vec<Post> =
        PostsManager::get_all_posts_by_slug(&mut conn, "tercer-post".to_string());

    for post in posts_filter_by_slug_result {
        println!("se creo el post con titulo: {}", post.title)
    }
}
