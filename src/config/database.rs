use mybatis::mybatis::Mybatis;
use mybatis::logic_delete::MyBatisLogicDeletePlugin;
use serde::{Serialize, Deserialize}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // 数据库地址
    pub database_url: String,
    // 数据库用户名
    pub database_username: String,
    // 数据库密码
    pub database_password: String,
}

pub fn init(config: &M) -> Mybatis {
    let mut conn = MyBatis::new();

    // 逻辑删除配置
    conn.logic_plugin = Some(Box::new(MybatisLogicDeletePlugin::new_opt(
        &config.logic_column,
        config.logic_deleted as i32,
        config.logic_un_deleted as i32,
    )));
    
    conn
}

impl Config {
    fn new(database_url: String, database_username: String, database_password: String) -> Self{
        let properties = include_str!("../resource/application.yml");
        let result: Config = serde_yaml::from_str(properties).expect("load config file fail");

        result
    }
}