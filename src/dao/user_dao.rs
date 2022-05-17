use crate::config::database;
use crate::domain;
use mybatis::crud::CRUD;
use domain::entity::user::User;
use mybatis::snowflake;

#[async_trait]
pub trait UserDao {
    async fn user_register(&self);
}

#[async_trait]
impl UserDao for User{

    async fn user_register(&self) {
        let mybatis = database::conn().await;
    
        mybatis.save(&self,&[]).await.unwrap();
    }
}

