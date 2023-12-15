#[derive(Debug, Clone, Default)]
pub enum IncomeType {
    #[default]
    None,
    Salary,
    HourlyWage,
    ClassWage,
    LessonWage,
}
