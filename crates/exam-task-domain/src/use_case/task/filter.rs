use crate::{
    model::{FilterTask, Task},
    repository::{TaskRepository, TaskResult},
};

pub struct FilterTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    task_repository: TaskRepo,
}

impl<TaskRepo> FilterTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    pub fn new(task_repository: TaskRepo) -> Self {
        Self { task_repository }
    }

    pub async fn filter_task(&self, filter: FilterTask) -> TaskResult<Vec<Task>> {
        self.task_repository.filter(filter).await
    }
}
