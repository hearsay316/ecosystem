use axum::extract::Request;

use axum::routing::get;

use axum::Router;
use opentelemetry::trace::TracerProvider;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;

use opentelemetry_sdk::trace::{Config, RandomIdGenerator, TracerProvider as OtherTracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use std::time::Duration;
use time::macros::format_description;
use tokio::join;
use tokio::net::TcpListener;
use tokio::time::{sleep, Instant};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, Layer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //秒
    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0)?,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );
    // tracing_subscriber::fmt::init();
    let file_appender = tracing_appender::rolling::daily("./tem/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_timer(local_time.clone())
        .with_filter(LevelFilter::DEBUG);

    let file = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_timer(local_time)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    let tracer = init_tracer()?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer.tracer("default1"));
    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .with(opentelemetry)
        .init();
    let addr = "0.0.0.0:9000";
    let app = Router::new().route("/", get(index_handler));
    info!("Starting  server on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    /*
    service
    into_make_service
    into_make_serive
    */
    Ok(())
}
#[instrument(fields(http.uri = req.uri().path(),http.method = req.method().as_str()))]
async fn index_handler(_req: Request) -> &'static str {
    debug!(" index handler started");
    sleep(Duration::from_millis(10)).await;
    // let ret = long_task().await;
    // task1().await;
    // task2().await;
    // task3().await;
    let (ret, _, _, _) = join!(long_task(), task1(), task2(), task3());
    info!(http.status_code = 200, "index handler Request completed");
    ret
}
#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    let dur = 112;
    sleep(Duration::from_millis(dur)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task long  too long");
    "hello world"
}
#[instrument]
async fn task1() {
    sleep(Duration::from_millis(100)).await;
}
#[instrument]
async fn task2() {
    sleep(Duration::from_millis(100)).await;
}
#[instrument]
async fn task3() {
    sleep(Duration::from_millis(100)).await;
}
fn init_tracer() -> anyhow::Result<OtherTracerProvider> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            Config::default()
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(32)
                .with_max_attributes_per_span(64)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(runtime::Tokio)?;

    Ok(tracer)
}
