use std::str::FromStr;

use actix_web::{
    delete, get,
    http::header::ContentType,
    post,
    web::{Data, Path},
    HttpResponse,
};
use chrono::{NaiveDateTime, Utc};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    Insertable, PgConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::likes;
use crate::common::get_connection;
use crate::diesel::RunQueryDsl;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "likes"]
pub struct Like {
    id: Uuid,
    created_at: NaiveDateTime,
    tweet_id: Uuid,
}

impl Like {
    fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}

#[get("/tweets/{id}/likes")]
pub async fn get_tweet_likes_by_id(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // TODO: Get tweet likes by id
    use crate::schema::likes::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection(pool);

    if let Ok(path_id) = Uuid::from_str(&path.0) {
        let result = likes
            .filter(tweet_id.eq_all(path_id))
            .load::<Like>(&connection)
            .expect("Error loading tweet likes");

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&result)
    } else {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json("Tweet id not found")
    }
}

#[post("/tweets/{id}/likes")]
pub async fn set_like_to_tweed(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    let connection = get_connection(pool);

    if let Ok(path_id) = Uuid::from_str(&path.0) {
        let new_like = Like::new(path_id);

        diesel::insert_into(likes::table)
            .values(&new_like)
            .execute(&connection)
            .expect("Error to add new like");

        HttpResponse::Created()
            .content_type(ContentType::json())
            .json(&new_like)
    } else {
        HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json("Tweet id not found")
    }
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like_to_tweed(
    path: Path<(String,)>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> HttpResponse {
    // TODO: Delete tweet like by id

    use crate::schema::likes::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection(pool);

    if let Ok(path_id) = Uuid::from_str(&path.0) {
        diesel::delete(likes.filter(tweet_id.eq_all(path_id)))
            .execute(&connection)
            .expect("Error to delete tweet like");

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
