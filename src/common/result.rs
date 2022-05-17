use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::result_code::ResultCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IResult<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> IResult<T>
{   
    pub fn new(code: String, msg: String, data: Option<T>) -> Self {
        Self {
            code,
            msg,
            data,
        }
    }

    pub fn success() -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::SUCCESS),
            msg: ResultCode::get_msg(ResultCode::SUCCESS),
            data: None,
        }
    }

    pub fn success_data(data: T) -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::SUCCESS),
            msg: ResultCode::get_msg(ResultCode::SUCCESS),
            data: Some(data),
        }
    }

    pub fn error() -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::ERROR),
            msg: ResultCode::get_msg(ResultCode::ERROR),
            data: None,
        }
    }
}

impl<T> ToString for IResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}