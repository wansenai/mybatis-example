use serde::{Deserialize, Serialize};
use mybatis::DateTimeNative;

// 用户注册 
#[crud_table]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Option<String>,
    // 用户名
    pub user_name: Option<String>,
    // 用户密码
    pub user_password: Option<String>,
    // 姓名
    pub name: Option<String>,
    // 电话
    pub phone: Option<String>,
    // 性别 0- 女 1-男
    pub sex: Option<i32>,
    // 创建时间
    pub create_time: Option<DateTimeNative>,
    // 状态 0- 启用 1-停用
    pub status: Option<u8>,
}