use crate::config::database;
use crate::domain;
use mybatis::crud::CRUD;
use domain::entity::user::User;
use mybatis::snowflake;

#[async_trait]
pub trait UserDao {
    async fn user_register(&self);

    async fn exist_user(&self) -> bool;
}

#[async_trait]
impl UserDao for User{

    async fn user_register(&self) {
        let mybatis = database::conn().await;
    
        mybatis.save(&self,&[]).await.unwrap();
    }

    async fn exist_user(&self) -> bool {
        let mybatis = database::conn().await;

        let result: Option<User> = mybatis.fetch_by_column("user_name", &self.user_name).await.unwrap();
        println!("query object: {:?}",result);
        
        let mut bool_flag = false;

        match result {
            Some(User) => bool_flag = true,
            None => (),
        };

        bool_flag
    }
}

