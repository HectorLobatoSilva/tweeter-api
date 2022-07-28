use std::str::FromStr;

use actix_web::{
    delete, get,
    http::header::ContentType,
    post,
    web::{self, Data, Path},
    HttpResponse, Responder,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Insertable, PgConnection, Queryable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::tweets;
use crate::common::get_connection;
use crate::diesel::RunQueryDsl;

#[derive(Deserialize)]
pub struct Body {
    message: String,
}

#[derive(Insertable, Queryable, Deserialize, Serialize, PartialEq)]
#[table_name = "tweets"]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    // TODO: Get all tweets
    use crate::schema::tweets::dsl::*;
    let connection = get_connection(pool);

    let results = tweets.load::<Tweet>(&connection);
    let response = match results {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(response)
}

#[post("/tweets")]
pub async fn create_tweets(
    request: web::Json<Body>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // TODO: Create new tweet

    let new_tweet = Tweet::new(request.message.clone());
    let connection = get_connection(pool);

    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .execute(&connection)
        .expect("Error creating new tweet");

    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(&new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // TODO: Get tweet by id
    use crate::schema::tweets::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection(pool);

    if let Ok(path_id) = Uuid::from_str(&path.0) {
        let result = tweets
            .filter(id.eq_all(path_id))
            .first::<Tweet>(&connection)
            .expect("Error loading tweet");

        // let tweets = format!("This is {:?} tweet", path.0);
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&result)
    } else {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json("Tweet id not found")
    }
}

#[delete("/tweets/{id}")]
pub async fn delete_tweet_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    // TODO: Delete tweet by id
    use crate::schema::tweets::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection(pool);

    if let Ok(path_id) = Uuid::from_str(&path.0) {
        diesel::delete(tweets.filter(id.eq_all(path_id)))
            .execute(&connection)
            .expect("Error to delete tweet");

        HttpResponse::NoContent()
            .content_type(ContentType::json())
            .await
            .unwrap()
    } else {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json("Tweet id not found")
    }
}
