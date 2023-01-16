use reqwest::Client;
use shaku::{Component, HasComponent, Module};

use crate::schedule::Scheduler;

pub struct SchedulerComponent(());

impl<M> Component<M> for SchedulerComponent
where
    M: Module + HasComponent<Client>,
{
    type Interface = Scheduler;

    type Parameters = ();

    fn build(
        context: &mut shaku::ModuleBuildContext<M>,
        _: Self::Parameters,
    ) -> Box<Self::Interface> {
        let client = (*M::build_component(context)).clone();
        let scheduler = Scheduler::new(client);
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
