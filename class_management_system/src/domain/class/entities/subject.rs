use crate::domain::{
    class::shared::entities::Exam,
    shared::{user::entities::Teacher, values::Currency},
};

use super::Lesson;

pub struct Subject {
    id: String,
    title: String,
    description: String,
    lessons: Vec<Lesson>,
    level: i8,
    teacher: Teacher,
    fee: Currency,
    presentation_time: String,
    start_time: String,
    start_date: String,
    end_date: String,
    exams: Vec<Exam>,
}
