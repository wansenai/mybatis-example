use mybatis::mybatis::Mybatis;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub driver_type: String,
    // 数据库地址
    pub database_url: String,
    // 数据库用户名
    pub database_username: String,
    // 数据库密码
    pub database_password: String,
}

impl Config {
    pub fn get_connection_str() -> String{
        let mut mybatis = Mybatis::new();

        let properties = include_str!("../resource/application.yml");
        let result: Config = serde_yaml::from_str(properties).expect("load config file fail");

        if String::from("com.mysql.jdbc.Driver").eq(&result.driver_type) {
            return format!(
                "mysql://{}:{}@{}", result.database_username, result.database_password, result.database_url
            );
        } else {
            panic!("not support driver");
        }
        // 本示例先使用mysql 驱动
    }
}

pub async fn conn() -> Mybatis{

    let url = Config::get_connection_str();
    let mybatis = Mybatis::new();

    mybatis.link(&url).await.unwrap();

    mybatis
}