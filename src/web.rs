use crate::controllers;
use actix_http::{http::StatusCode, Response};
use actix_web::{middleware, web, App, HttpServer, Responder, ResponseError};
use std::convert::From;
use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum WebError {
    #[fail(display = "{}", _0)]
    BadRequest(String),
    #[fail(display = "auth failed")]
    AuthFailed,
    #[fail(display = "resouces not found")]
    NotFound,
    #[fail(display = "diesel error")]
    DieselError,
    #[fail(display = "database error")]
    DbError,
    #[fail(display = "validation error")]
    Hash,
    #[fail(display = "json format error")]
    JsonFormatError,
    #[fail(display = "json format error")]
    BcryptError,
}

impl ResponseError for WebError {
    fn error_response(&self) -> Response {
        Response::new(match self {
            WebError::NotFound => StatusCode::NOT_FOUND,
            WebError::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

pub type WebResult<T> = actix_http::error::Result<T, WebError>;

impl From<validator::ValidationErrors> for WebError {
    fn from(_err: validator::ValidationErrors) -> WebError {
        WebError::BadRequest(serde_json::to_string(&_err).unwrap())
    }
}
impl From<bcrypt::BcryptError> for WebError {
    fn from(_err: bcrypt::BcryptError) -> WebError {
        WebError::BcryptError
    }
}

impl From<r2d2::Error> for WebError {
    fn from(_err: r2d2::Error) -> WebError {
        WebError::DbError
    }
}
impl From<diesel::result::Error> for WebError {
    fn from(_err: diesel::result::Error) -> WebError {
        WebError::DieselError
    }
}

impl From<serde_json::Error> for WebError {
    fn from(_err: serde_json::Error) -> WebError {
        WebError::JsonFormatError
    }
}

pub fn run() -> io::Result<()> {
    let db_pool = crate::db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            // .route("/info/{name}", web::get().to_async(info))
            .route("/info/{id}", web::get().to(info2))
            .route("/register", web::post().to(controllers::user::register))
            .route("/login", web::post().to(controllers::user::login))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

pub fn info2(path: web::Path<i32>, db: web::Data<crate::db::PgPool>) -> impl Responder {
    let pool = db.get().unwrap();

    use crate::models::user::User;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    let results = users
        .filter(id.eq(path.into_inner()))
        .limit(1)
        .load::<User>(&*pool)?;
    WebResult::Ok(web::Json(results))
}
