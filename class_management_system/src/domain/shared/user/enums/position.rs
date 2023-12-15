#[derive(Debug, Clone)]
pub enum Position {
    Student,
    Teacher,
}

impl Position {
    /// Returns `true` if the position is [`Student`].
    ///
    /// [`Student`]: Position::Student
    #[must_use]
    pub fn is_student(&self) -> bool {
        matches!(self, Self::Student)
    }

    /// Returns `true` if the position is [`Teacher`].
    ///
    /// [`Teacher`]: Position::Teacher
    #[must_use]
    pub fn is_teacher(&self) -> bool {
        matches!(self, Self::Teacher)
    }
}