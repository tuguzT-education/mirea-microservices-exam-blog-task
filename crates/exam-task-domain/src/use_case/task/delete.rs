use crate::{
    model::{Task, TaskId},
    repository::{TaskRepository, TaskResult},
};

#[derive(Debug, Clone)]
pub struct DeleteTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    task_repository: TaskRepo,
}

impl<TaskRepo> DeleteTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    pub fn new(task_repository: TaskRepo) -> Self {
        Self { task_repository }
    }

    pub async fn delete_task(&self, id: TaskId) -> TaskResult<Task> {
        self.task_repository.delete(id).await
    }
}
