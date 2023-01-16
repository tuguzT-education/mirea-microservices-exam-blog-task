use std::collections::HashMap;

use chrono::Utc;
use exam_task_domain::model::{Task, TaskId};
use reqwest::Client;
use tokio::{
    sync::RwLock,
    task::{self, JoinHandle},
    time,
};

use crate::model::CreatePost;

#[derive(Debug)]
pub struct Scheduler {
    handles: RwLock<HashMap<TaskId, JoinHandle<()>>>,
    client: Client,
}

impl Scheduler {
    pub fn new(client: Client) -> Self {
        Self {
            handles: RwLock::default(),
            client,
        }
    }

    pub async fn publish_task(&self, task: Task) {
        let mut handles = self.handles.write().await;
        match task.date_to_publish {
            Some(date_to_publish) => {
                let id = task.id.clone();
                let client = self.client.clone();
                let duration = (date_to_publish - Utc::now()).to_std().ok();
                let handle = task::spawn(async move {
                    if let Some(duration) = duration {
                        time::sleep(duration).await;
                    }
                    let create = CreatePost {
                        name: task.name,
                        description: task.description,
                    };
                    let _response = client
                        .post("http://krakend:8080/api/post")
                        .json(&create)
                        .send()
                        .await
                        .unwrap();
                });
                handles.insert(id, handle);
            }
            None => {
                let handle = handles.remove(&task.id);
                if let Some(handle) = handle {
                    handle.abort();
                }
            }
        };
    }
}
