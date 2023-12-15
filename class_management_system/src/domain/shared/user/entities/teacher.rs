use crate::domain::shared::{
    user::enums::{CharacterPrefix, ContractType, IncomeType, Position, Specialty},
    values::{Currency, PhoneNumber},
};

#[derive(Debug, Clone,Default)]
pub struct Teacher {
    id: String,
    prefix: CharacterPrefix,
    name: String,
    middle_name: String,
    lastname: String,
    position: Position,
    email: String,
    phone_number: PhoneNumber,
    salary: Currency,
    specialty: Specialty,
    contract_type: ContractType,
    income_type: IncomeType,
}
