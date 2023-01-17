use std::{
    net::{SocketAddr, TcpListener},
    sync::Arc,
};

use axum::{Extension, Router, Server};
use exam_task::{
    di::app_module,
    model::{CreateTaskData, TaskData, UpdateTaskData},
    route::task,
};
use reqwest::{Client, StatusCode};

#[tokio::test]
async fn test_task() {
    let addr = "127.0.0.1:8081".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let database_uri = std::env::var("DATABASE_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let post_service_url = std::env::var("POST_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8082".to_string());
        let module = app_module(database_uri, post_service_url).build();
        let module = Arc::new(module);

        let app = Router::new().merge(task::all()).layer(Extension(module));
        Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = Client::new();

    let create = CreateTaskData {
        blog_id: "waltuh".to_string(),
        name: "Hello World".to_string(),
    };
    let response = client
        .post(format!("http://{addr}/"))
        .json(&create)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let task: TaskData = response.json().await.unwrap();
    assert_eq!(task.name, create.name);

    let response = client
        .get(format!("http://{addr}/{}", task.id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let found_task: TaskData = response.json().await.unwrap();
    assert_eq!(task, found_task);

    let update = UpdateTaskData::builder().name("Microservices").build();
    let response = client
        .post(format!("http://{addr}/{}", task.id))
        .json(&update)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let updated_task: TaskData = response.json().await.unwrap();
    assert_eq!(updated_task.id, task.id);
    assert_eq!(updated_task.name, update.name.unwrap());

    let response = client
        .delete(format!("http://{addr}/{}", task.id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let deleted_task: TaskData = response.json().await.unwrap();
    assert_eq!(deleted_task.id, task.id);

    let response = client
        .get(format!("http://{addr}/{}", task.id))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
