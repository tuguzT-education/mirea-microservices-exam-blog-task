use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_more::{Display, Error, From};
use exam_task_domain::repository::TaskError;
use serde_json::json;

#[derive(Debug, Display, From, Error)]
pub enum AppError {
    Task(TaskError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Task(err) => match err {
                TaskError::NoTaskFound => StatusCode::NOT_FOUND,
                TaskError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };
        let body = json!({
            "error": self.to_string(),
        });
        let body = Json(body);
        (status, body).into_response()
    }
}
