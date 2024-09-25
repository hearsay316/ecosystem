
use std::time::Duration;
use axum::Router;
use axum::routing::get;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::{Tracer, TracerProvider};
use time::macros::format_description;
use tokio::net::TcpListener;
use tokio::time::{sleep, Instant};
use tracing::{info, level_filters::LevelFilter, instrument, warn, debug};

use tracing_subscriber::{fmt, Layer};
use tracing_subscriber::fmt::format::FmtSpan;

use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    let opentelemetry =tracing_opentelemetry::layer().with_tracer(tracer.tracer()) ;
    tracing_subscriber::registry().with(console).with(file).with(opentelemetry).init();
    let addr = "0.0.0.0:9000";
    let app = Router::new().route(
        "/", get(index_handler),
    );
    info!("Starting  server on {}",addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    /*
    service
    into_make_service
    into_make_serive
    */
    Ok(())
}
#[instrument]
async fn index_handler() -> &'static str {
    debug!(" index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_task().await;
    info!(http.status = 200,"index handler Request completed");
    ret
}
#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    let dur = 112;
    sleep(Duration::from_millis(dur)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration=elapsed,"task long  too long");
    "hello world"
}
fn init_tracer()->anyhow::Result<TracerProvider>{
    let tracer =
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://locahost:4317")
        ).install_batch(runtime::Tokio)?;

    Ok(tracer)
}