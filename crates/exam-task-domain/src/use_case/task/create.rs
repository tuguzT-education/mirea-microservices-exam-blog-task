use crate::{
    model::{CreateTask, Task},
    repository::{IdRepository, TaskRepository, TaskResult},
};

#[derive(Debug, Clone)]
pub struct CreateTaskUseCase<TaskRepo, IdRepo>
where
    TaskRepo: TaskRepository,
    IdRepo: IdRepository,
{
    task_repository: TaskRepo,
    id_repository: IdRepo,
}

impl<TaskRepo, IdRepo> CreateTaskUseCase<TaskRepo, IdRepo>
where
    TaskRepo: TaskRepository,
    IdRepo: IdRepository,
{
    pub fn new(task_repository: TaskRepo, id_repository: IdRepo) -> Self {
        Self {
            task_repository,
            id_repository,
        }
    }

    pub async fn create_task(&self, create: CreateTask) -> TaskResult<Task> {
        let id = self.id_repository.create().await?;
        let task = Task {
            id: id.with_owner(),
            blog_id: create.blog_id,
            name: create.name,
            description: "".to_string(),
            is_closed: false,
            date_to_publish: None,
        };
        self.task_repository.create(task).await
    }
}
