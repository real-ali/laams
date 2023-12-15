#[derive(Debug, Clone, Default)]
pub enum ContractType {
    #[default]
    None,
    Permanent,
    Temporary,
    Freelance,
    Internship,
}
