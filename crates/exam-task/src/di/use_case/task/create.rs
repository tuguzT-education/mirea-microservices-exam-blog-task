use std::sync::Arc;

use exam_task_domain::use_case::CreateTaskUseCase;
use shaku::{Component, HasComponent, Module};

use crate::di::repository::{IdRepository, TaskRepository};

pub struct CreateTaskUseCaseComponent(());

impl<M> Component<M> for CreateTaskUseCaseComponent
where
    M: Module + HasComponent<dyn TaskRepository> + HasComponent<dyn IdRepository>,
{
    type Interface = CreateTaskUseCase<Arc<dyn TaskRepository>, Arc<dyn IdRepository>>;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let task_repository = M::build_component(context);
        let id_repository = M::build_component(context);

        let use_case = CreateTaskUseCase::new(task_repository, id_repository);
        Box::new(use_case)
    }
}
