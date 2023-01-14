use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_more::{Display, Error, From};

use crate::model::{FilterTask, InternalError, Task, TaskId};

#[async_trait]
#[auto_impl(&, Box, Arc)]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, task: Task) -> Result<Task>;

    async fn read(&self, id: TaskId) -> Result<Task>;

    async fn filter(&self, filter: FilterTask) -> Result<Vec<Task>>;

    async fn update(&self, task: Task) -> Result<Task>;

    async fn delete(&self, id: TaskId) -> Result<Task>;
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From, Error)]
pub enum Error {
    #[from(ignore)]
    #[display(fmt = "no task found")]
    NoTaskFound,
    #[display(fmt = "internal error")]
    Internal(InternalError),
}
