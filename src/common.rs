use actix_web::web::Data;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub fn get_connection(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Cant get database conecction")
}
