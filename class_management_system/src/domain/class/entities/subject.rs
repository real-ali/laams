use crate::domain::{
    class::shared::entities::Exam,
    shared::{user::entities::Teacher, values::Currency},
};

use super::Lesson;
#[derive(Debug, Clone, Default)]
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

impl Subject {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn get_title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn get_description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn get_lessons(&self) -> &[Lesson] {
        self.lessons.as_ref()
    }

    pub fn get_level(&self) -> i8 {
        self.level
    }

    pub fn get_teacher(&self) -> &Teacher {
        &self.teacher
    }

    pub fn get_fee(&self) -> &Currency {
        &self.fee
    }

    pub fn get_presentation_time(&self) -> &str {
        self.presentation_time.as_ref()
    }

    pub fn get_start_time(&self) -> &str {
        self.start_time.as_ref()
    }

    pub fn get_start_date(&self) -> &str {
        self.start_date.as_ref()
    }

    pub fn get_end_date(&self) -> &str {
        self.end_date.as_ref()
    }

    pub fn get_exams(&self) -> &[Exam] {
        self.exams.as_ref()
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_lessons(mut self, lessons: Vec<Lesson>) -> Self {
        self.lessons = lessons;
        self
    }

    pub fn set_level(mut self, level: i8) -> Self {
        self.level = level;
        self
    }

    pub fn set_teacher(mut self, teacher: Teacher) -> Self {
        self.teacher = teacher;
        self
    }

    pub fn set_fee(mut self, fee: Currency) -> Self {
        self.fee = fee;
        self
    }

    pub fn set_presentation_time(mut self, presentation_time: String) -> Self {
        self.presentation_time = presentation_time;
        self
    }

    pub fn set_start_time(mut self, start_time: String) -> Self {
        self.start_time = start_time;
        self
    }

    pub fn set_start_date(mut self, start_date: String) -> Self {
        self.start_date = start_date;
        self
    }

    pub fn set_end_date(mut self, end_date: String) -> Self {
        self.end_date = end_date;
        self
    }

    pub fn set_exams(mut self, exams: Vec<Exam>) -> Self {
        self.exams = exams;
        self
    }
}

