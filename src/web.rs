use crate::controllers;
use actix_web::{middleware, web, App, HttpServer, Responder};
use std::io;

pub fn run() -> io::Result<()> {
    let db_pool = crate::db::create_connection_pool();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            // .route("/info/{name}", web::get().to_async(info))
            .route("/info/{name}", web::get().to(info2))
            .route("/register", web::post().to(controllers::user::register))
            .route("/login", web::post().to(controllers::user::login))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::intrinsics::type_name::<T>());
}
pub fn info2(path: web::Path<i32>, db: web::Data<crate::db::PgPool>) -> impl Responder {
    let pool = db.get().unwrap();

    use crate::models::user::User;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    let results = users
        // .filter(email.eq("zxk7516@foxmail.com".to_string()))
        .filter(id.eq(path.into_inner()))
        .limit(1)
        .load::<User>(&*pool)
        .expect("Error loading user");
    web::Json(results)
}
