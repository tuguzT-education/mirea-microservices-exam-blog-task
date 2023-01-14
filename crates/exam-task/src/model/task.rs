use chrono::{DateTime, Utc};
use exam_task_domain::model::{CreateTask, FilterTask, Task, UpdateTask};
use serde::{Deserialize, Serialize};

use super::utils::deserialize_optional_field;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TaskData {
    pub id: String,
    pub post_id: Option<String>,
    pub name: String,
    pub description: String,
    pub is_closed: bool,
    pub date_to_publish: Option<DateTime<Utc>>,
}

impl From<Task> for TaskData {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.into(),
            post_id: task.post_id.map(Into::into),
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish,
        }
    }
}

impl From<TaskData> for Task {
    fn from(task: TaskData) -> Self {
        Self {
            id: task.id.into(),
            post_id: task.post_id.map(Into::into),
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CreateTaskData {
    pub name: String,
}

impl From<CreateTask> for CreateTaskData {
    fn from(create: CreateTask) -> Self {
        Self { name: create.name }
    }
}

impl From<CreateTaskData> for CreateTask {
    fn from(create: CreateTaskData) -> Self {
        Self { name: create.name }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FilterTaskData {
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<Option<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_closed: Option<bool>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to_publish: Option<Option<DateTime<Utc>>>,
}

impl From<FilterTaskData> for FilterTask {
    fn from(filter: FilterTaskData) -> Self {
        Self {
            id: filter.id.map(Into::into),
            post_id: filter.post_id.map(|it| it.map(Into::into)),
            name: filter.name,
            description: filter.description,
            is_closed: filter.is_closed,
            date_to_publish: filter.date_to_publish,
        }
    }
}

impl From<FilterTask> for FilterTaskData {
    fn from(filter: FilterTask) -> Self {
        Self {
            id: filter.id.map(Into::into),
            post_id: filter.post_id.map(|it| it.map(Into::into)),
            name: filter.name,
            description: filter.description,
            is_closed: filter.is_closed,
            date_to_publish: filter.date_to_publish,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateTaskData {
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<Option<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_closed: Option<bool>,
    #[serde(deserialize_with = "deserialize_optional_field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to_publish: Option<Option<DateTime<Utc>>>,
}

impl From<UpdateTask> for UpdateTaskData {
    fn from(update: UpdateTask) -> Self {
        Self {
            post_id: update.post_id.map(|it| it.map(Into::into)),
            name: update.name,
            description: update.description,
            is_closed: update.is_closed,
            date_to_publish: update.date_to_publish,
        }
    }
}

impl From<UpdateTaskData> for UpdateTask {
    fn from(update: UpdateTaskData) -> Self {
        Self {
            post_id: update.post_id.map(|it| it.map(Into::into)),
            name: update.name,
            description: update.description,
            is_closed: update.is_closed,
            date_to_publish: update.date_to_publish,
        }
    }
}
