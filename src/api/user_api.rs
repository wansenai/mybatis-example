use actix_web::{post, web, Responder};

use crate::domain::dto::user::UserRegisterDto;
use crate::service::user_service;
use crate::common::result::IResult;

///
/// user register
/// 
#[post("/user/register")]
pub async fn register(arg: web::Json<UserRegisterDto>) -> impl Responder {
    log::info!("UserRegisterDto: {:?}", arg.0);
    user_service::save_user(&arg).await;

    let response: IResult<String> = IResult::success();
    web::Json(response)
}