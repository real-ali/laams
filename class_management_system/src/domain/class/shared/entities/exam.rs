
use crate::domain::class::shared::enums::ExamType;

use super::{ExamQuestion, ExamResultsSheet};

pub struct Exam {
    id: String,
    questions: Vec<ExamQuestion>,
    exam_type: ExamType,
    passing_score: i8,
    max_score: i8,
    results_sheets: Vec<ExamResultsSheet>,
}
