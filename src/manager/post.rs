use crate::models::post::{NewPost, Post};
use crate::schema::posts::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;

pub struct PostsManager;

impl PostsManager {
    // Método para insertar un nuevo post
    pub fn create_new_post(conn: &mut PgConnection, new_post: NewPost) -> Post {
        diesel::insert_into(posts::table())
            .values(&new_post)
            .get_result(conn)
            .expect("error insertando registro")
    }

    // Método para obtener todos los posts
    pub fn get_all_posts(conn: &mut PgConnection) -> Vec<Post> {
        posts.load::<Post>(conn).expect("error al ejecutar la Query posts load")
    }

    // Método para obtener post basado en su slug
    pub fn get_all_posts_by_slug(conn: &mut PgConnection, filter_slug: String) -> Vec<Post> {
        posts.filter(slug.eq(filter_slug)).load::<Post>(conn).expect("error al ejecutar la Query posts load")
    }
}
