use crate::db::PgPool;
use crate::models::user::User;
use crate::requests::user::{LoginForm, RegisterForm};
use crate::schema::users::{self, dsl::*};
use crate::web::{WebError, WebResult};
use actix_web::{web, Responder};
use diesel::prelude::*;
use validator::Validate;

pub fn login(db: web::Data<PgPool>, form: web::Json<LoginForm>) -> impl Responder {
    let form = form.into_inner();
    let _ = form.validate()?;
    let login = users
        .filter(name.eq(form.name))
        .limit(1)
        .load::<User>(&*db.get().unwrap())?;
    let login = login.first().ok_or(WebError::NotFound)?;
    if bcrypt::verify(form.password, &login.password)
        .map_err(|_| WebError::InternalServerError)?
    {
        WebResult::Ok(serde_json::to_string(login)?)
    } else {
        WebResult::Err(WebError::AuthFailed)
    }
}

pub fn register(db: web::Data<PgPool>, form: web::Json<RegisterForm>) -> impl Responder {
    let mut form = form.into_inner();
    let _ = form.validate()?;

    form.password = bcrypt::hash(form.password, 4)?;
    let r = diesel::insert_into(users::table)
        .values(&form)
        .get_result::<crate::models::user::User>(&*db.get().unwrap())?;
    crate::web::WebResult::Ok(serde_json::to_string(&r)?)
}

pub fn info2(path: web::Path<i32>, db: web::Data<crate::db::PgPool>) -> impl Responder {
    let login = users
        .filter(id.eq(path.into_inner()))
        .limit(1)
        .load::<User>(&*db.get().unwrap())?;
    WebResult::Ok(serde_json::to_string(&login))
}
