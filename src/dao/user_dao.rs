use crate::config::database;
use crate::domain;

use domain::dto::user::{
    UserRegisterDto,
};
use mybatis::crud::CRUD;
use domain::entity::user::UserRegister;


#[async_trait]
pub trait UserDao {
    async fn user_register(&self, user_dto: &UserRegisterDto);
}

#[async_trait]
impl UserDao for UserRegister{

    async fn user_register(&self, user_dto: &UserRegisterDto) {

        println!("come in");

        let mybatis = database::conn().await;

        println!("in execution");

        let user = UserRegister {
            id: user_dto.id.clone(),
            user_name: user_dto.name.clone(),
            user_password: user_dto.user_password.clone(),
            name: user_dto.name.clone(),
            phone: user_dto.phone.clone(),
        };

        println!("{:?}", user);
    
        mybatis.save(&user,&[]).await.unwrap();
    }
}

