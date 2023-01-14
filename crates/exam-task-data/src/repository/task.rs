use async_trait::async_trait;
use exam_task_domain::{
    model::{FilterTask, Task, TaskId},
    repository::TaskResult,
};

use crate::data_source::TaskDataSource;

mod domain {
    pub use exam_task_domain::repository::TaskRepository;
}

pub struct TaskRepository {
    data_source: TaskDataSource,
}

impl TaskRepository {
    pub fn new(data_source: TaskDataSource) -> Self {
        Self { data_source }
    }
}

#[async_trait]
impl domain::TaskRepository for TaskRepository {
    async fn create(&self, task: Task) -> TaskResult<Task> {
        self.data_source.create(task).await
    }

    async fn read(&self, id: TaskId) -> TaskResult<Task> {
        self.data_source.read(id).await
    }

    async fn filter(&self, filter: FilterTask) -> TaskResult<Vec<Task>> {
        self.data_source.filter(filter).await
    }

    async fn update(&self, task: Task) -> TaskResult<Task> {
        self.data_source.update(task).await
    }

    async fn delete(&self, id: TaskId) -> TaskResult<Task> {
        self.data_source.delete(id).await
    }
}
