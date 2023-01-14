use async_trait::async_trait;
use exam_task_domain::{
    model::{Task, TaskId},
    repository::{TaskRepository, TaskResult},
};

use crate::data_source::TaskDataSource;

pub struct TaskRepositoryImpl {
    data_source: TaskDataSource,
}

impl TaskRepositoryImpl {
    pub fn new(data_source: TaskDataSource) -> Self {
        Self { data_source }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn create(&self, task: Task) -> TaskResult<Task> {
        self.data_source.create(task).await
    }

    async fn read(&self, id: TaskId) -> TaskResult<Task> {
        self.data_source.read(id).await
    }

    async fn update(&self, task: Task) -> TaskResult<Task> {
        self.data_source.update(task).await
    }

    async fn delete(&self, id: TaskId) -> TaskResult<Task> {
        self.data_source.delete(id).await
    }
}
