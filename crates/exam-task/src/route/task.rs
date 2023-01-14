use axum::{
    extract::Path,
    routing::{delete, get, post},
    Json, Router,
};
use shaku_axum::Inject;

use crate::{
    di::{AppModule, CreateTaskUseCase, DeleteTaskUseCase, ReadTaskUseCase, UpdateTaskUseCase},
    model::{CreateTaskData, TaskData, UpdateTaskData},
    route::error::AppError,
};

pub fn all() -> Router {
    Router::new()
        .merge(create_task())
        .merge(read_tasks())
        .merge(update_task())
        .merge(delete_task())
}

pub fn create_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, CreateTaskUseCase>,
        Json(create): Json<CreateTaskData>,
    ) -> Result<Json<TaskData>, AppError> {
        let create = create.into();
        let task = use_case.create_task(create).await?.into();
        Ok(Json(task))
    }

    Router::new().route("/", post(handler))
}

pub fn read_tasks() -> Router {
    async fn handler(
        use_case: Inject<AppModule, ReadTaskUseCase>,
        Path(id): Path<String>,
    ) -> Result<Json<TaskData>, AppError> {
        let id = id.into();
        let task = use_case.read_task(id).await?.into();
        Ok(Json(task))
    }

    Router::new().route("/:id", get(handler))
}

pub fn update_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, UpdateTaskUseCase>,
        Path(id): Path<String>,
        Json(update): Json<UpdateTaskData>,
    ) -> Result<Json<TaskData>, AppError> {
        let id = id.into();
        let update = update.into();
        let task = use_case.update_task(id, update).await?.into();
        Ok(Json(task))
    }

    Router::new().route("/:id", post(handler))
}

pub fn delete_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, DeleteTaskUseCase>,
        Path(id): Path<String>,
    ) -> Result<Json<TaskData>, AppError> {
        let id = id.into();
        let task = use_case.delete_task(id).await?.into();
        Ok(Json(task))
    }

    Router::new().route("/:id", delete(handler))
}
