use crate::{
    application::common::persistence::TaskRepository,
    infrastructure::controllers::task::TaskController,
};

pub struct AppState {
    pub task_controller: TaskController,
}
