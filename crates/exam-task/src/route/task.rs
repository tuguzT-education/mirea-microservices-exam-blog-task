use axum::{
    extract::{Path, Query},
    routing::{delete, get, post},
    Json, Router,
};
use shaku_axum::Inject;

use crate::{
    di::{
        AppModule, CreateTaskUseCase, DeleteTaskUseCase, FilterTaskUseCase, ReadTaskUseCase,
        UpdateTaskUseCase,
    },
    model::{CreateTaskData, FilterTaskData, TaskData, UpdateTaskData},
    route::error::AppError,
    schedule::Scheduler,
};

pub fn all() -> Router {
    Router::new()
        .merge(create_task())
        .merge(read_task())
        .merge(filter_task())
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

pub fn read_task() -> Router {
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

pub fn filter_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, FilterTaskUseCase>,
        Query(filter): Query<FilterTaskData>,
    ) -> Result<Json<Vec<TaskData>>, AppError> {
        let filter = filter.into();
        let tasks = use_case.filter_task(filter).await?;
        let tasks = tasks.into_iter().map(Into::into).collect();
        Ok(Json(tasks))
    }

    Router::new().route("/", get(handler))
}

pub fn update_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, UpdateTaskUseCase>,
        scheduler: Inject<AppModule, Scheduler>,
        Path(id): Path<String>,
        Json(update): Json<UpdateTaskData>,
    ) -> Result<Json<TaskData>, AppError> {
        let id = id.into();
        let update = update.into();
        let task = use_case.update_task(id, update).await?;
        scheduler.set_publish_task_state(task.clone()).await;

        let task = task.into();
        Ok(Json(task))
    }

    Router::new().route("/:id", post(handler))
}

pub fn delete_task() -> Router {
    async fn handler(
        use_case: Inject<AppModule, DeleteTaskUseCase>,
        scheduler: Inject<AppModule, Scheduler>,
        Path(id): Path<String>,
    ) -> Result<Json<TaskData>, AppError> {
        let id = id.into();
        let task = use_case.delete_task(id).await?;
        scheduler.set_publish_task_state(task.clone()).await;

        let task = task.into();
        Ok(Json(task))
    }

    Router::new().route("/:id", delete(handler))
}
