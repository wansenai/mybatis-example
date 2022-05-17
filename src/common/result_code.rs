use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultCode {
    SUCCESS,
    ERROR,
}

#[allow(dead_code)]
impl ResultCode{
    pub fn get_code(result: ResultCode) -> String {
        let mut code = String::new();

        match result {
            SUCCESS => code = String::from("200"),
            Error => code = String::from("500"),
        };

        code
    }

    pub fn get_msg(result: ResultCode) -> String {
        let mut msg = String::new();

        match result {
            SUCCESS => msg = String::from("成功"),
            Error => msg = String::from("失败"),
        };

        msg
    }
}