use serde::{Deserialize, Serialize};
use mybatis::DateTimeNative;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRole {
    // id
    pub id: Option<String>,
    // user id
    pub user_id: Option<String>,
    // role id
    pub role_id: Option<String>,
    // create time
    pub create_time: Option<DateTimeNative>,
    // status
    pub status: Option<i32>,
}