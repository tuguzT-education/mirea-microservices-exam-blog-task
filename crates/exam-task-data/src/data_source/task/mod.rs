use exam_task_domain::{
    model::{InternalError, Task, TaskId},
    repository::{TaskError, TaskResult},
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneAndReplaceOptions, ReturnDocument},
    Collection,
};

pub use model::TaskData;

use super::Client;

mod model;

pub struct TaskDataSource {
    collection: Collection<TaskData>,
}

impl TaskDataSource {
    pub fn new(client: Client) -> Self {
        let collection = client.task_collection();
        Self { collection }
    }

    pub async fn create(&self, task: Task) -> TaskResult<Task> {
        let data = TaskData::try_from(task.clone()).map_err(InternalError::new)?;
        let _result = self
            .collection
            .insert_one(data, None)
            .await
            .map_err(InternalError::new)?;
        Ok(task)
    }

    pub async fn read(&self, id: TaskId) -> TaskResult<Task> {
        let id = ObjectId::parse_str(id.to_string()).map_err(InternalError::new)?;
        let filter = doc! { "_id": id };
        let data = self
            .collection
            .find_one(filter, None)
            .await
            .map_err(InternalError::new)?
            .ok_or(TaskError::NoTaskFound)?;
        let task = Task::from(data);
        Ok(task)
    }

    pub async fn update(&self, task: Task) -> TaskResult<Task> {
        let id = ObjectId::parse_str(task.id.to_string()).map_err(InternalError::new)?;
        let filter = doc! { "_id": id };
        let replacement = TaskData::try_from(task).map_err(InternalError::new)?;
        let options = FindOneAndReplaceOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
        let data = self
            .collection
            .find_one_and_replace(filter, replacement, options)
            .await
            .map_err(InternalError::new)?
            .ok_or(TaskError::NoTaskFound)?;
        let task = Task::from(data);
        Ok(task)
    }

    pub async fn delete(&self, id: TaskId) -> TaskResult<Task> {
        let id = ObjectId::parse_str(id.to_string()).map_err(InternalError::new)?;
        let query = doc! { "_id": id };
        let data = self
            .collection
            .find_one_and_delete(query, None)
            .await
            .map_err(InternalError::new)?
            .ok_or(TaskError::NoTaskFound)?;
        let task = Task::from(data);
        Ok(task)
    }
}
