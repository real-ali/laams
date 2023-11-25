use crate::domain::common::{enums::CharacterPrefix, values::PhoneNumber};

#[derive(Debug,Clone)]
pub struct  User{
    pub id : String,
    pub prefix: CharacterPrefix,
    pub name : String,
    pub middle_name: String,
    pub lastname : String,
    pub email : String,
    pub phone_number : PhoneNumber
}