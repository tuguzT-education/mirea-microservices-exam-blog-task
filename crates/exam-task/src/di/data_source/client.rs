use anyhow::Context;
use exam_task_data::data_source::Client;
use futures::executor::block_on;
use shaku::{Component, HasComponent, Module};
use tokio::runtime::Handle;

pub struct ClientComponent(());

impl<M> Component<M> for ClientComponent
where
    M: Module + HasComponent<DatabaseUri>,
{
    type Interface = Client;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let conn_str = &*M::build_component(context).0;

        let handle = Handle::current();
        let _ = handle.enter();
        let client = block_on(Client::new(conn_str))
            .with_context(|| "tried to create local data source client")
            .unwrap();

        Box::new(client)
    }
}

pub struct DatabaseUri(pub String);

impl<M> Component<M> for DatabaseUri
where
    M: Module,
{
    type Interface = Self;

    type Parameters = Option<String>;

    fn build(
        _: &mut shaku::ModuleBuildContext<M>,
        params: Self::Parameters,
    ) -> Box<Self::Interface> {
        let database_uri = params.expect("Database URI should be set");
        Box::new(Self(database_uri))
    }
}
