use crate::domain::{class::shared::enums::ExamType, shared::user::entities::Student};

use super::{ExamQuestion, ExamResultsSheet};
#[derive(Debug, Clone, Default)]
pub struct Exam {
    id: String,
    questions: Vec<ExamQuestion>,
    exam_type: ExamType,
    passing_score: i8,
    max_score: i8,
    results_sheets: Vec<ExamResultsSheet>,
}

impl Exam {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn get_questions(&self) -> &[ExamQuestion] {
        self.questions.as_ref()
    }

    pub fn get_exam_type(&self) -> &ExamType {
        &self.exam_type
    }

    pub fn get_passing_score(&self) -> i8 {
        self.passing_score
    }

    pub fn get_max_score(&self) -> i8 {
        self.max_score
    }

    pub fn get_results_sheets(&self) -> &[ExamResultsSheet] {
        self.results_sheets.as_ref()
    }

    pub fn set_questions(mut self, questions: Vec<ExamQuestion>) -> Self {
        self.questions = questions;
        self
    }

    pub fn set_exam_type(mut self, exam_type: ExamType) -> Self {
        self.exam_type = exam_type;
        self
    }

    pub fn set_passing_score(mut self, passing_score: i8) -> Self {
        self.passing_score = passing_score;
        self
    }

    pub fn set_max_score(mut self, max_score: i8) -> Self {
        self.max_score = max_score;
        self
    }

    pub fn set_results_sheets(mut self, results_sheets: Vec<ExamResultsSheet>) -> Self {
        self.results_sheets = results_sheets;
        self
    }
}

impl Exam {
    pub fn get_passed_students(&self) -> Vec<Student> {
        let mut students = Vec::<Student>::new();
        let _ = self.get_results_sheets().into_iter().map(|result| {
            let is_passed = result.get_score() >= self.get_passing_score();
            if is_passed {
                students.push(result.get_examinee().clone())
            }
        });
        students
    }
    pub fn get_faild_students(&self) -> Vec<Student> {
        let mut students = Vec::<Student>::new();
        let _ = self.get_results_sheets().into_iter().map(|result| {
            let is_faild = result.get_score() < self.get_passing_score();
            if is_faild {
                students.push(result.get_examinee().clone())
            }
        });
        students
    }
}
