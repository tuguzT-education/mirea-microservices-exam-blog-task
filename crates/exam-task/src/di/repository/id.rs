use std::sync::Arc;

use shaku::{Component, Interface, Module};

mod domain {
    pub use exam_task_domain::repository::IdRepository;
}

mod data {
    pub use exam_task_data::repository::IdRepository;
}

pub trait IdRepository: domain::IdRepository + Interface {
    fn upcast(self: Arc<Self>) -> Arc<dyn domain::IdRepository>;
}

impl<T> IdRepository for T
where
    T: domain::IdRepository + Interface,
{
    fn upcast(self: Arc<Self>) -> Arc<dyn domain::IdRepository> {
        self
    }
}

pub struct IdRepositoryComponent(());

impl<M> Component<M> for IdRepositoryComponent
where
    M: Module,
{
    type Interface = dyn IdRepository;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        let id_repository = data::IdRepository::default();
        Box::new(id_repository)
    }
}
