use serde::{Deserialize, Serialize};
use mybatis::DateTimeNative;

// 用户注册 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegisterDto {
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<i32>,
    pub create_time: Option<DateTimeNative>,
}

/// 用户角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

/// 用户角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleEditDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}