use std::sync::{Arc, Mutex};
use axum::{Json, Router};
use axum::extract::State;
use axum::routing::{get, patch};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{debug, info, instrument};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, Layer};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    skills: Vec<String>,
}

#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
struct UserUpdate {

    age: Option<u32>,
    skills: Option<Vec<String>>,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();
    let layer = fmt::Layer::new().pretty().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();


    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "javascript".to_string()],
    };
    let user = Arc::new(Mutex::new(user));
    let app = Router::new()

        .route("/",get(index_handler))
        .route("/",patch(update_handler))
        .with_state(user);
    let addr = "0.0.0.0:9000";
    info!("Starting  server on {}",addr);
    info!(" 服务开始");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener,app).await?;
    Ok(())
}
#[instrument]
async fn index_handler(State(user):State<Arc<Mutex<User>>>)->Json<User>{
    debug!(" index handler started");
    // let user = User {
    //     name: "Alice".to_string(),
    //     age: 30,
    //     skills: vec!["Rust".to_string(), "javascript".to_string()],
    // };
    // let json = serde_json::to_string(&user).unwrap();
    // println!("{json:?}");
    //
    // let user1:User = serde_json::from_str(&json).unwrap();
    // println!("{:?}",user1);
    // assert_eq!(user1,user);
    Json::from((*user.lock().unwrap()).clone())

}
#[instrument]
async fn update_handler(State(user):State<Arc<Mutex<User>>>,Json(user_update):Json<UserUpdate>)->Json<User>{

let  mut user = user.lock().unwrap();
    if let Some(age) = user_update.age{
        user.age = age;
    }

    if let Some(skills) = user_update.skills{
        user.skills = skills;
    }
    (*user).clone().into()
}