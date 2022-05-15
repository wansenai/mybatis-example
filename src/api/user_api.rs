use actix_web::{post, web, HttpRequest, Responder};

use crate::domain::dto::user::UserRegisterDto;
use crate::service::user_service;


/// 用户登陆
#[post("/user/register")]
pub async fn login(arg: web::Json<UserRegisterDto>) -> impl Responder {
    log::info!("UserRegister:{:?}", arg.0);
    user_service::save_user(&arg).await;

    format!("Hello")
}