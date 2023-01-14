use async_trait::async_trait;
use exam_task_domain::{model::ErasedId, repository::IdResult};
use mongodb::bson::oid::ObjectId;

mod domain {
    pub use exam_task_domain::repository::IdRepository;
}

#[derive(Debug, Default)]
pub struct IdRepository(());

#[async_trait]
impl domain::IdRepository for IdRepository {
    async fn create(&self) -> IdResult<ErasedId> {
        let id = ObjectId::new().to_string().into();
        Ok(id)
    }
}
