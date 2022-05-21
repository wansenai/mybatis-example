use serde::{Deserialize, Serialize};
use mybatis::DateTimeNative;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    // id
    pub id: Option<String>,
    // role id
    pub role_code: Option<String>,
    // role name
    pub role_name: Option<String>,
    // role status
    pub role_status: Option<i32>,
    // create time
    pub create_time: Option<DateTimeNative>,
    // update time
    pub update_time: Option<DateTimeNative>,
}