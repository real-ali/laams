use crate::domain::shared::{user::enums::{CharacterPrefix, Position}, values::PhoneNumber};


#[derive(Debug, Clone,Default)]
pub struct Student {
    id: String,
    prefix: CharacterPrefix,
    name: String,
    middle_name: String,
    lastname: String,
    position: Position,
    email: String,
    phone_number: PhoneNumber,
}
