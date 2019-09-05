use crate::db::PgPool;
use crate::requests::user::RegisterForm;
use crate::schema::users;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use validator::Validate;

pub fn login() -> impl Responder {
    ""
}

pub fn register(db: web::Data<PgPool>, form: web::Json<RegisterForm>) -> impl Responder {
    // form.into_inner()
    const JSON_CONTENT_TYPE: &str = "application/json";
    let mut form = form.into_inner();
    match form.validate() {
        Err(e) => {
            return HttpResponse::BadRequest()
                .content_type(JSON_CONTENT_TYPE)
                .body(serde_json::to_string(&e).unwrap())
        }
        Ok(_) => {
            form.password = "".to_string();
            let r = diesel::insert_into(users::table)
                .values(&form)
                .get_result::<crate::models::user::User>(&*db.get().unwrap())
                .expect("error register user");
            HttpResponse::Ok()
                .content_type(JSON_CONTENT_TYPE)
                .body(serde_json::to_string(&r).unwrap())
            // web::Json(r)
        }
    }
}
