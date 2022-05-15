use crate::dao::user_dao::UserDao;

use crate::domain::dto::user::UserRegisterDto;
use crate::domain::entity::user::UserRegister;

pub async fn save_user(user_dto: &UserRegisterDto) {
    println!("userdto: {:?}", user_dto);

    let a = UserRegister{
        id: Some(String::from("55")),
        user_name: Some(String::from("55")),
        user_password: Some(String::from("55")),
        name: Some(String::from("55")),
        phone: Some(String::from("55")),
    };

    UserDao::user_register(&a, &user_dto).await;
}