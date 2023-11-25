use std::{borrow::BorrowMut, collections::HashMap, sync::Mutex};

use crate::{
    application::common::{persistence::TaskRepository, AppResult},
    domain::task::Task,
};

pub struct TaskModel {
    id: String,
    title: String,
    description: String,
}

impl From<Task> for TaskModel {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
        }
    }
}
pub struct TaskRepositoryLocal {
    data: Mutex<HashMap<String, TaskModel>>,
}

impl TaskRepositoryLocal {
    pub fn new() -> Self {
        Self {
            data: Mutex::default(),
        }
    }
}

impl TaskRepository for TaskRepositoryLocal {
    fn save_task(&self, task: Task) -> AppResult<()> {
        println!("Repository -> {:<20}", "TaskRepository.save_task");
        let mut borrow = self.data.lock().unwrap();
        borrow.insert(task.id.clone(), task.into());
        Ok(())
    }
}
