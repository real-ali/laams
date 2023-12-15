use crate::domain::shared::user::entities::Student;

#[derive(Debug, Clone, Default)]
pub struct ExamResultsSheet {
    id: String,
    examinee: Student,
    score: i8,
}

impl ExamResultsSheet {
    pub fn new(id: String, examinee: Student, score: i8) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn get_examinee(&self) -> &Student {
        &self.examinee
    }

    pub fn get_score(&self) -> i8 {
        self.score
    }

    pub fn set_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn set_examinee(mut self, examinee: Student) -> Self {
        self.examinee = examinee;
        self
    }

    pub fn set_score(mut self, score: i8) -> Self {
        self.score = score;
        self
    }
}
