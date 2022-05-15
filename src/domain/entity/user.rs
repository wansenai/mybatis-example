use serde::{Deserialize, Serialize};

// 用户注册 
#[crud_table]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegister {
    pub id: Option<String>,
    // 用户名
    pub user_name: Option<String>,
    // 用户密码
    pub user_password: Option<String>,
    // 姓名
    pub name: Option<String>,
    // 电话
    pub phone: Option<String>,
}