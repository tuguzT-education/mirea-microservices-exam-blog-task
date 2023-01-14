use exam_task_data::data_source::{Client, TaskDataSource};
use shaku::{Component, HasComponent, Module};

pub struct TaskDataSourceComponent(());

impl<M> Component<M> for TaskDataSourceComponent
where
    M: Module + HasComponent<Client>,
{
    type Interface = TaskDataSource;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let client = (*M::build_component(context)).clone();
        let data_source = TaskDataSource::new(client);
        Box::new(data_source)
    }
}
