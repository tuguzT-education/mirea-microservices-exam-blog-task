use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use exam_task_domain::model::{Task, TaskId, UpdateTask};
use reqwest::Client;
use tokio::{
    sync::RwLock,
    task::{self, JoinHandle},
    time,
};

use crate::{di::UpdateTaskUseCase, model::CreatePost};

#[derive(Clone)]
pub struct Scheduler {
    handles: Arc<RwLock<HashMap<TaskId, JoinHandle<()>>>>,
    client: Client,
    update: UpdateTaskUseCase,
}

impl Scheduler {
    pub fn new(client: Client, update: UpdateTaskUseCase) -> Self {
        Self {
            handles: Default::default(),
            client,
            update,
        }
    }

    pub async fn publish_task(&self, task: Task) {
        match task.date_to_publish {
            Some(date_to_publish) => {
                let id = task.id.clone();
                let handles = self.handles.clone();
                let client = self.client.clone();
                let update_use_case = self.update.clone();

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

                    let mut handles = handles.write().await;
                    handles.remove(&task.id);

                    let update = UpdateTask::builder()
                        .is_closed(true)
                        .date_to_publish(None)
                        .build();
                    update_use_case.update_task(task.id, update).await.unwrap();
                });
                let mut handles = self.handles.write().await;
                handles.insert(id, handle);
            }
            None => {
                let mut handles = self.handles.write().await;
                if let Some(handle) = handles.remove(&task.id) {
                    handle.abort();
                }
            }
        }
    }
}
