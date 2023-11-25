use crate::{
    application::common::{persistence::TaskRepository, AppResult},
    domain::{
        common::{models::Aggregate, DomainError},
        task::Task,
    },
};

pub struct CreateTaskCommand {
    pub title: String,
    pub description: String,
}

pub struct CreateTaskCommandHandler<'a> {
    rep: &'a dyn TaskRepository,
}

impl<'a> CreateTaskCommandHandler<'a> {
    pub fn new(rep: &'a dyn TaskRepository) -> Self {
        Self { rep }
    }
}

impl<'a> CreateTaskCommandHandler<'a> {
    pub async fn execute(&self, cmd: CreateTaskCommand) -> AppResult<String> {
        println!("Command -> {:>20}", "CreateTaskCommandHandler");

        let task = Task::new("13434", cmd.title).set_description(cmd.description);
        if !task.validate() {
            return Err(DomainError::new("InvalidDomain"));
        }

        self.rep.save_task(task.clone());

        Ok(String::from(task.id.clone()))
    }
}
