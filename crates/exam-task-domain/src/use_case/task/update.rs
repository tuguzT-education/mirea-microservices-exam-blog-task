use crate::{
    model::{Task, TaskId, UpdateTask},
    repository::{TaskRepository, TaskResult},
};

pub struct UpdateTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    task_repository: TaskRepo,
}

impl<TaskRepo> UpdateTaskUseCase<TaskRepo>
where
    TaskRepo: TaskRepository,
{
    pub fn new(task_repository: TaskRepo) -> Self {
        Self { task_repository }
    }

    pub async fn update_task(&self, id: TaskId, update: UpdateTask) -> TaskResult<Task> {
        let task = self.task_repository.read(id).await?;
        let task = Task {
            id: task.id,
            post_id: update.post_id.unwrap_or(task.post_id),
            name: update.name.unwrap_or(task.name),
            description: update.description.unwrap_or(task.description),
            is_closed: update.is_closed.unwrap_or(task.is_closed),
            date_to_publish: update.date_to_publish.unwrap_or(task.date_to_publish),
        };
        self.task_repository.update(task).await
    }
}
