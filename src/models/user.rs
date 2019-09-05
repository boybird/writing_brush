use serde::Serialize;


#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}
