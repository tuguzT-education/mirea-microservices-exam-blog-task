use std::sync::Arc;

use exam_task_data::data_source::TaskDataSource;
use shaku::{Component, HasComponent, Interface, Module};

mod domain {
    pub use exam_task_domain::repository::TaskRepository;
}

mod data {
    pub use exam_task_data::repository::TaskRepository;
}

pub trait TaskRepository: domain::TaskRepository + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn domain::TaskRepository>;
}

impl<T> TaskRepository for T
where
    T: domain::TaskRepository + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn domain::TaskRepository> {
        self
    }
}

pub struct TaskRepositoryComponent(());

impl<M> Component<M> for TaskRepositoryComponent
where
    M: Module + HasComponent<TaskDataSource>,
{
    type Interface = dyn TaskRepository;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let data_source = (*M::build_component(context)).clone();
        Box::new(data::TaskRepository::new(data_source))
    }
}
