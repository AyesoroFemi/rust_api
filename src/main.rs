// use axum::{routing::get, Router};


// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(|| async {"Hello, Rust"}));

//     println!("Running on http://localhost:3000");
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap()
// }
// fn main() {
//     println!("Hello, world!");
// }


use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router
};

use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String
}

async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

async fn list_users() -> Json<Vec<User>> {
    let users = ec! [
        User {
            id: 1,
            name: "Elijah".to_string(),
            email: "elijah@example.com".to_string()
        },
         User {
            id: 2,
            name: "John".to_string(),
            email: "john@example.com".to_string()
        },
    ];
    Json(users)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello, Rust first time"}))
        .route("/create-user", post(create_user))
        .route("/users", get(list_users));

        println!("Running on http://localhost:3000");
        axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
}