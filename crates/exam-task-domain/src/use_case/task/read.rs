use crate::{
    model::{Task, TaskId},
    repository::{TaskRepository, TaskResult},
};

#[derive(Debug, Clone)]
pub struct ReadTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    task_repository: TaskRepo,
}

impl<TaskRepo> ReadTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    pub fn new(task_repository: TaskRepo) -> Self {
        Self { task_repository }
    }

    pub async fn read_task(&self, id: TaskId) -> TaskResult<Task> {
        self.task_repository.read(id).await
    }
}
