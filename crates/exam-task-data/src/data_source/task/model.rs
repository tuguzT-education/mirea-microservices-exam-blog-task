use exam_task_domain::model::Task;
use mongodb::bson::{
    oid::{Error, ObjectId},
    DateTime,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TaskData {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub blog_id: String,
    pub name: String,
    pub description: String,
    pub is_closed: bool,
    pub date_to_publish: Option<DateTime>,
}

impl From<TaskData> for Task {
    fn from(task: TaskData) -> Self {
        Self {
            id: task.id.to_string().into(),
            blog_id: task.blog_id.into(),
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish.map(DateTime::to_chrono),
        }
    }
}

impl TryFrom<Task> for TaskData {
    type Error = Error;

    fn try_from(task: Task) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ObjectId::parse_str(task.id.to_string())?,
            blog_id: task.blog_id.into(),
            name: task.name,
            description: task.description,
            is_closed: task.is_closed,
            date_to_publish: task.date_to_publish.map(DateTime::from_chrono),
        })
    }
}
