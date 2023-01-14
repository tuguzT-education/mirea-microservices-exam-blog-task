use std::sync::Arc;

use exam_task_domain::use_case::ReadTaskUseCase;
use shaku::{Component, HasComponent, Module};

use crate::di::repository::TaskRepository;

pub struct ReadTaskUseCaseComponent(());

impl<M> Component<M> for ReadTaskUseCaseComponent
where
    M: Module + HasComponent<dyn TaskRepository>,
{
    type Interface = ReadTaskUseCase<Arc<dyn TaskRepository>>;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let task_repository = M::build_component(context);
        let use_case = ReadTaskUseCase::new(task_repository);
        Box::new(use_case)
    }
}
