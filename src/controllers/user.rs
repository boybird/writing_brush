use crate::db::PgPool;
use crate::models::user::User;
use crate::requests::user::{LoginForm, RegisterForm};
use crate::schema::users::{self, dsl::*};
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use validator::Validate;

pub fn login(db: web::Data<PgPool>, form: web::Json<LoginForm>) -> impl Responder {
    let form = form.into_inner();
    match form.validate() {
        Err(e) => HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&e).unwrap()),
        Ok(_) => {
            match users
                .filter(name.eq(form.name))
                .limit(1)
                .load::<User>(&*db.get().unwrap())
            {
                Ok(results) => {
                    match results.first() {
                        Some(_i) => {
                            if form.password == _i.password {
                                // TODO generate jwt web token
                                HttpResponse::Ok()
                                    .content_type("application/json")
                                    .body(serde_json::to_string(_i).unwrap())
                            } else {
                                HttpResponse::Unauthorized()
                                    .content_type("application/json")
                                    .body("{\"message\":\"password missmatch.\"}")
                            }
                        }
                        None => HttpResponse::Unauthorized()
                            .content_type("application/json")
                            .body("{\"message\":\"user does not exits.\"}"),
                    }
                }
                _ => HttpResponse::Unauthorized().finish(),
            }
        }
    }
}

pub fn register(db: web::Data<PgPool>, form: web::Json<RegisterForm>) -> impl Responder {
    // form.into_inner()
    let mut form = form.into_inner();
    match form.validate() {
        Err(e) => {
            return HttpResponse::BadRequest()
                .content_type("application/json")
                .body(serde_json::to_string(&e).unwrap())
        }
        Ok(_) => {
            form.password = "".to_string();
            let r = diesel::insert_into(users::table)
                .values(&form)
                .get_result::<crate::models::user::User>(&*db.get().unwrap())
                .expect("error register user");
            HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&r).unwrap())
            // web::Json(r)
        }
    }
}
