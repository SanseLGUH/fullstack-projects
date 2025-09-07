use actix_web::{
    web, post, HttpResponse, Responder
};

use sqlx::{Connection, PgConnection, FromRow};
use sqlx::postgres::PgPool;

#[derive(FromRow)]
struct Profile {
}

pub async fn login() {
}

pub async fn delete_profile() {
}