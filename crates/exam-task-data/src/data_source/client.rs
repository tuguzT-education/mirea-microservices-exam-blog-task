use mongodb::{error::Result, options::ClientOptions, Client as MongoClient, Collection, Database};

use super::task::TaskData;

#[derive(Debug, Clone)]
pub struct Client(MongoClient);

impl Client {
    pub async fn new(conn_str: impl AsRef<str>) -> Result<Self> {
        let mut client_options = ClientOptions::parse(conn_str).await?;
        client_options.app_name = Some("exam-task".to_string());

        let client = MongoClient::with_options(client_options)?;
        Ok(Self(client))
    }

    pub(super) fn task_collection(&self) -> Collection<TaskData> {
        self.database().collection("task")
    }

    fn database(&self) -> Database {
        self.0.database("exam-task")
    }
}
