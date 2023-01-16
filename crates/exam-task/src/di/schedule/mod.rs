use std::sync::Arc;

use reqwest::Client;
use shaku::{Component, HasComponent, Module};

use crate::schedule::Scheduler;

use super::UpdateTaskUseCase;

pub struct SchedulerComponent(());

impl<M> Component<M> for SchedulerComponent
where
    M: Module + HasComponent<Client> + HasComponent<UpdateTaskUseCase>,
{
    type Interface = Scheduler;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let client: Arc<Client> = M::build_component(context);
        let client = (*client).clone();

        let update: Arc<UpdateTaskUseCase> = M::build_component(context);
        let update = (*update).clone();

        let scheduler = Scheduler::new(client, update);
        Box::new(scheduler)
    }
}

pub struct ClientComponent(());

impl<M> Component<M> for ClientComponent
where
    M: Module,
{
    type Interface = Client;

    type Parameters = ();

    fn build(_: &mut shaku::ModuleBuildContext<M>, _: Self::Parameters) -> Box<Self::Interface> {
        let client = Client::new();
        Box::new(client)
    }
}
