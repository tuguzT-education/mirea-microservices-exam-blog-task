use std::sync::Arc;

use exam_task_domain::use_case::FilterTaskUseCase;
use shaku::{Component, HasComponent, Module};

use crate::di::repository::TaskRepository;

pub struct FilterTaskUseCaseComponent(());

impl<M> Component<M> for FilterTaskUseCaseComponent
where
    M: Module + HasComponent<dyn TaskRepository>,
{
    type Interface = FilterTaskUseCase<Arc<dyn TaskRepository>>;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let task_repository = M::build_component(context);
        let use_case = FilterTaskUseCase::new(task_repository);
        Box::new(use_case)
    }
}
