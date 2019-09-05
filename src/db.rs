use diesel::pg::PgConnection;
use diesel::prelude::*;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type PgConnectionManager = ConnectionManager<PgConnection>;
pub type PgPool = Pool<PgConnectionManager>;

pub fn create_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("")
}

pub fn create_connection_pool() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PgConnectionManager::new(database_url);
    let pool = PgPool::builder()
        .build(manager)
        .expect("Failed to create bool.");

    pool
}

#[cfg(test)]
mod tests {
    use super::create_connection_pool;
    use diesel::prelude::*;
    #[test]
    fn test_pg_connection() {}

    #[test]
    fn test_pg_connection_pool() {
        use crate::models::user::User;
        use crate::schema::users::dsl::*;
        let pool = create_connection_pool();
        let conn = &*pool.get().unwrap();
        let results = users
            // .filter(email.eq("zxk7516@foxmail.com".to_string()))
            .filter(id.eq(1))
            .limit(1)
            .load::<User>(conn)
            .expect("Error loading user");
        for user in results {
            println!("{:?}", user);
        }
    }
}
