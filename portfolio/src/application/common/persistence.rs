use crate::domain::task::Task;

use super::AppResult;

pub trait TaskRepository {
    fn save_task(&self, task: Task) -> AppResult<()>;
}
