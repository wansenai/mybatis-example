use actix_web::{post, web, Responder};

use crate::domain::dto::user::UserRegisterDto;
use crate::service::user_service;
use crate::common::result::IResult;
use crate::common::result_code::ResultCode;

///
/// user register
/// 
#[post("/user/register")]
pub async fn register(arg: web::Json<UserRegisterDto>) -> impl Responder {
    log::info!("UserRegisterDto: {:?}", arg.0);
    let result = user_service::save_user(&arg).await;

    if result == true {
        let success_response: IResult<String> = IResult::success();
        web::Json(success_response)
    } else {

        let code = ResultCode::get_code(ResultCode::USER_ALREADY_EXIST);
        let msg = ResultCode::get_msg(ResultCode::USER_ALREADY_EXIST);

        let error_response: IResult<String> = IResult::other_error(&code, &msg);
        web::Json(error_response)
    }
}