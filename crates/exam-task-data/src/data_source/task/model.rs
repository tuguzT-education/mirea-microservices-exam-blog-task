use chrono::{DateTime, Utc};
use exam_task_domain::model::Task;
use mongodb::bson::oid::{Error, ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TaskData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub post_id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub is_closed: bool,
    pub date_to_publish: Option<DateTime<Utc>>,
}

impl From<TaskData> for Task {
    fn from(task: TaskData) -> Self {
        Self {
            id: task.id.to_string().into(),
            post_id: task.post_id.map(|id| id.to_string().into()),
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish,
        }
    }
}

impl TryFrom<Task> for TaskData {
    type Error = Error;

    fn try_from(task: Task) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ObjectId::parse_str(task.id.to_string())?,
            post_id: task
                .post_id
                .map(|id| ObjectId::parse_str(id.to_string()))
                .transpose()?,
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish,
        })
    }
}
