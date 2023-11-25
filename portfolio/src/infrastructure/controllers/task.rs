use crate::{
    application::{
        common::persistence::TaskRepository,
        task::commands::create::{CreateTaskCommand, CreateTaskCommandHandler},
    },
    infrastructure::persistence::TaskRepositoryLocal,
    presentation::rest::result::RestResult,
};

pub struct TaskController {
    pub repo: TaskRepositoryLocal,
}

pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
}

impl Into<CreateTaskCommand> for CreateTaskRequest {
    fn into(self) -> CreateTaskCommand {
        CreateTaskCommand {
            title: self.title,
            description: self.description,
        }
    }
}

impl TaskController {
    pub async fn create_task(&self, req: CreateTaskRequest) -> RestResult<String> {
        println!("Controller -> {:<40}", "create_task");
        let command = CreateTaskCommandHandler::new(&self.repo);
        let id = command.execute(req.into()).await?;
        Ok(id)
    }
}
