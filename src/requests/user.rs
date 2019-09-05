extern crate validator;

use serde::Deserialize;

use validator::Validate;
// use validate::traits::Validate;

use crate::schema::users;

#[derive(Deserialize, Debug, Insertable, Validate)]
#[table_name = "users"]
pub struct RegisterForm {
    #[validate(length(min = 5, max = 32, message = "用户名长度为(5-32)"))]
    pub name: String,
    #[validate(email(message = "邮件格式不正确"))]
    pub email: String,
    #[validate(length(min = 6, max = 50, message = "密码长度为(6-50)"))]
    pub password: String,
}
