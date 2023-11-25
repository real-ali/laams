use actix_web::{
    post,
    web::{self, Json, ServiceConfig},
    HttpResponse, Responder,
};
use serde_json::{json, Value};

use crate::{
    application::common::persistence::TaskRepository,
    infrastructure::controllers::task::CreateTaskRequest, presentation::rest::result::RestResult,
};

use super::state::AppState;

pub fn task_router(cfg: &mut ServiceConfig) {
    cfg.service(create_task_handler);
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateTaskPayload {
    title: String,
    description: String,
}

impl Into<CreateTaskRequest> for CreateTaskPayload {
    fn into(self) -> CreateTaskRequest {
        CreateTaskRequest {
            title: self.title,
            description: self.description,
        }
    }
}

#[post("/tasks")]
pub async fn create_task_handler(
    state: web::Data<AppState>,
    Json(payload): Json<CreateTaskPayload>,
) -> RestResult<Json<Value>> {
    println!("Handler -> {:<40}", "create_task_handler");
    let id = state.task_controller.create_task(payload.into()).await?;
    Ok(Json(json!(id)))
}
