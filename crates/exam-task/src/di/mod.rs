use std::sync::Arc;

use anyhow::Result;
use shaku::module;

use self::{
    data_source::DatabaseUrl,
    repository::{IdRepository, TaskRepository},
};

mod data_source;
mod repository;
mod use_case;

module! {
    pub AppModule {
        components = [
            data_source::DatabaseUrl,
            data_source::ClientComponent,
            data_source::TaskDataSourceComponent,
            repository::TaskRepositoryComponent,
            repository::IdRepositoryComponent,
            use_case::CreateTaskUseCaseComponent,
            use_case::DeleteTaskUseCaseComponent,
            use_case::ReadTaskUseCaseComponent,
            use_case::UpdateTaskUseCaseComponent,
        ],
        providers = [],
    }
}

pub fn app_module(database_url: String) -> Result<AppModule> {
    let module = AppModule::builder()
        .with_component_parameters::<DatabaseUrl>(database_url)
        .build();
    Ok(module)
}

pub type CreateTaskUseCase =
    domain::CreateTaskUseCase<Arc<dyn TaskRepository>, Arc<dyn IdRepository>>;

pub type ReadTaskUseCase = domain::ReadTaskUseCase<Arc<dyn TaskRepository>>;

pub type UpdateTaskUseCase = domain::UpdateTaskUseCase<Arc<dyn TaskRepository>>;

pub type DeleteTaskUseCase = domain::DeleteTaskUseCase<Arc<dyn TaskRepository>>;

mod domain {
    pub use exam_task_domain::use_case::{
        CreateTaskUseCase, DeleteTaskUseCase, ReadTaskUseCase, UpdateTaskUseCase,
    };
}
