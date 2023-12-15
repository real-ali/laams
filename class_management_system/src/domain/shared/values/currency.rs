use crate::domain::shared::enums::CurrencyType;

#[derive(Debug, Clone)]
pub struct Currency {
    currency_type: CurrencyType,
    value: i32,
}
