mod entities;
mod enums;
mod values;

use self::{
    enums::{Priority, Status},
    values::Timeline,
};

use super::common::{models::{Aggregate, Entity}, entities::User};

#[derive(Debug, Default, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub people: Vec<User>,
    pub doc: String, // path
    pub timeline: Timeline,
    pub connection: Vec<Task>,
    pub status: Status,
    pub items: Vec<Task>,
}

impl Task {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        println!("Domain -> Task.new");
        Self {
            id: id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Entity<String> for Task {
    fn validate(&self) -> bool {
        match self.id.is_empty() {
            true => false,
            false => true,
        }
    }
}

impl Aggregate<String> for Task {
    fn validate(&self) -> bool {
        match self.id.is_empty() {
            true => false,
            false => true,
        }
    }
}
