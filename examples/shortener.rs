use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::future::BoxFuture;
use futures::FutureExt;
use http::header::LOCATION;
use http::{HeaderMap, StatusCode};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;

const LISTEN_ADDR: &str = "localhost:9876";
static mut A: i32 = 0;
#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}
#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, Clone, FromRow)]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}
impl UrlRecord {
    fn try_new() -> Self {
        Self {
            id: "-1".to_string(),
            url: "-1".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub(crate) enum MyError {
    #[error("Custom error :{0}")]
    Custom(String),
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        match self {
            MyError::NotFound => (StatusCode::NOT_FOUND, self.to_string()).into_response(),
            MyError::Custom(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            // 根据需要处理其他错误类型
        }
    }
}
impl AppState {
    async fn try_new(url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(url)
            .await
            .map_err(|e| MyError::Custom(e.to_string()))?;
        //创建 db
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS shortener (
            id CHAR(6) PRIMARY KEY,
            url TEXT NOT NULL UNIQUE
            )
    "#,
        )
        .execute(&pool)
        .await
        .map_err(|e| MyError::Custom(e.to_string()))?;
        Ok(Self { db: pool })
    }
    fn shorten<'a>(&'a self, url: &'a str) -> BoxFuture<anyhow::Result<String>> {
        async move {

            unsafe {
                let mut  id = "8zLn31".to_string();
                if A != 0{
                    id = nanoid!(6);
                }
                let mut ids:UrlRecord =   sqlx::query_as("INSERT INTO urls (id, url) VALUES ($1, $2) ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id")
                    .bind(&id)
                    .bind(url)
                    .fetch_one(&self.db)
                    .await.unwrap_or(UrlRecord::try_new());
                info!("{:?} ---A", ids);
                if ids.id== *"-1"{
                     A += 1;
                    ids = UrlRecord{
                        id:  self.shorten(url).await?,
                        url:"".to_string()
                    }
                }
                Ok(ids.id)
            }
        }.boxed()
    }
    async fn get_url(&self, id: &str) -> anyhow::Result<String> {
        let record: UrlRecord = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        println!("{:?}", record);
        Ok(record.url)
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_init();
    let listener = TcpListener::bind(&LISTEN_ADDR).await?;
    info!("Listener on :{}", LISTEN_ADDR);
    let url = "postgres://postgres:123321@127.0.0.1:5432/shortener";
    let state = AppState::try_new(url).await?;
    info!(" Connected to database :{}", url);
    let app = Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect))
        .with_state(state);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(()) //into_make_service
}
// body 需要在最后以后一个参数
async fn shorten(
    State(state): State<AppState>,
    Json(data): Json<ShortenReq>,
) -> anyhow::Result<impl IntoResponse, MyError> {
    let id: String = state
        .shorten(&data.url)
        .await
        .map_err(|_| MyError::NotFound)?;
    let body = Json(ShortenRes {
        url: format!("http://{}/{}", LISTEN_ADDR, id),
    });
    Ok((StatusCode::CREATED, body))
}
async fn redirect(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    info!("这个是测试 {}", id);
    let url = state
        .get_url(&id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut header = HeaderMap::new();
    header.insert(LOCATION, url.parse().unwrap());
    info!("到了泽瑞");
    Ok((StatusCode::FOUND, header))
}
fn log_init() {
    // let console_layer = console_subscriber::spawn();
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
}
