use serde::{Deserialize, Serialize};

// 用户注册 
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegisterDto {
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub user_password: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
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