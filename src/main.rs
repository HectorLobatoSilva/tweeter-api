#[macro_use]
extern crate diesel;

mod common;
mod likes;
mod schema;
mod tweets;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;

use actix_web::{web, App, HttpServer};

// Routes
// tweets - > GET, POST
// tweets/:id -> GET, DELETE
// tweets/:id/likes -> RESOURCE

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not founded");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Cant create conections pool");
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .service(tweets::get_tweets)
                .service(tweets::create_tweets)
                .service(tweets::get_tweet_by_id)
                .service(tweets::delete_tweet_by_id)
                .service(likes::get_tweet_likes_by_id)
                .service(likes::set_like_to_tweed)
                .service(likes::remove_like_to_tweed),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
