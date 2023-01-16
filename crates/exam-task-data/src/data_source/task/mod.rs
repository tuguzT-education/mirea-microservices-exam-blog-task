use exam_task_domain::{
    model::{FilterTask, InternalError, Task, TaskId},
    repository::{TaskError, TaskResult},
};
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document, Regex},
    options::{FindOneAndReplaceOptions, ReturnDocument},
    Collection,
};

pub use model::TaskData;

use super::Client;

mod model;

#[derive(Debug, Clone)]
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

    pub async fn filter(&self, filter_task: FilterTask) -> TaskResult<Vec<Task>> {
        let mut filter = Document::new();
        if let Some(id) = filter_task.id {
            let id = ObjectId::parse_str(id.to_string()).map_err(InternalError::new)?;
            filter.insert("_id", id);
        }
        if let Some(blog_id) = filter_task.blog_id {
            let blog_id = blog_id.to_string();
            filter.insert("blog_id", blog_id);
        }
        if let Some(name) = filter_task.name {
            let regex = Regex {
                pattern: name,
                options: "i".to_string(),
            };
            filter.insert("name", regex);
        }
        if let Some(description) = filter_task.description {
            let regex = Regex {
                pattern: description,
                options: "i".to_string(),
            };
            filter.insert("description", regex);
        }
        if let Some(is_closed) = filter_task.is_closed {
            filter.insert("is_closed", is_closed);
        }
        if let Some(date_to_publish) = filter_task.date_to_publish {
            filter.insert("date_to_publish", date_to_publish);
        }
        let cursor = self
            .collection
            .find(filter, None)
            .await
            .map_err(InternalError::new)?;
        let data = cursor
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(InternalError::new)?;
        let tasks = data.into_iter().map(Into::into).collect();
        Ok(tasks)
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
