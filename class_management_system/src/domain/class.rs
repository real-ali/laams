use self::entities::Subject;

use super::shared::user::entities::Student;

pub mod entities;
pub mod enums;
pub mod shared;
pub mod values;

#[derive(Debug, Clone, Default)]
pub struct Class {
    id: String,
    title: String,
    description: String,
    level: i8,
    subjects: Vec<Subject>,
    students: Vec<Student>,
    start_date: String,
    expiry_date: String,
}

impl Class {
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

    pub fn get_level(&self) -> i8 {
        self.level
    }

    pub fn get_subjects(&self) -> &[Subject] {
        self.subjects.as_ref()
    }

    pub fn get_students(&self) -> &[Student] {
        self.students.as_ref()
    }

    pub fn get_start_date(&self) -> &str {
        self.start_date.as_ref()
    }

    pub fn get_expiry_date(&self) -> &str {
        self.expiry_date.as_ref()
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_level(mut self, level: i8) -> Self {
        self.level = level;
        self
    }

    pub fn set_subjects(mut self, subjects: Vec<Subject>) -> Self {
        self.subjects = subjects;
        self
    }

    pub fn set_students(mut self, students: Vec<Student>) -> Self {
        self.students = students;
        self
    }

    pub fn set_start_date(mut self, start_date: String) -> Self {
        self.start_date = start_date;
        self
    }

    pub fn set_expiry_date(mut self, expiry_date: String) -> Self {
        self.expiry_date = expiry_date;
        self
    }
}
