use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultCode {
    SUCCESS,
    ERROR,
    USER_ALREADY_EXIST,
}

#[allow(dead_code)]
impl ResultCode{
    pub fn get_code(result: ResultCode) -> String {
        let mut code = String::new();

        match result {
            ResultCode::SUCCESS => code = String::from("200"),
            ResultCode::ERROR => code = String::from("500"),
            ResultCode::USER_ALREADY_EXIST => code = String::from("3000"),
        };

        code
    }

    pub fn get_msg(result: ResultCode) -> String {
        let mut msg = String::new();

        match result {
            ResultCode::SUCCESS => msg = String::from("成功"),
            ResultCode::ERROR => msg = String::from("失败"),
            ResultCode::USER_ALREADY_EXIST => msg = String::from("用户已被注册"),
        };

        msg
    }
}