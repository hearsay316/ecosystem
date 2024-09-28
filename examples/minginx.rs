use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;
#[allow(unused_imports)]
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
#[allow(unused_imports)]
use tracing_subscriber::Layer as _;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = resolve_config();
    let config = Arc::new(config);
    let layer = Layer::new().pretty();
    tracing_subscriber::registry().with(layer).init();
    info!("开始啦 :{:?}", config);
    info!("Upstream is {}", config.upstream_addr);
    info!("Listen :{}", config.listen_addr);
    let listener = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        let clone_config = Arc::clone(&config);
        info!("Accept connection from :{}", addr);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&clone_config.upstream_addr)
                .await
                .expect("三四收");
            proxy(client, upstream).await?;
            Ok::<(), anyhow::Error>(())
        });
    }
    #[allow(unreachable_code)]
    Ok::<(), anyhow::Error>(())
}
async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> anyhow::Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();
    let client_to_upstream = tokio::io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = tokio::io::copy(&mut upstream_reader, &mut client_writer);
    match tokio::try_join!(client_to_upstream, upstream_to_client) {
        Ok((n, m)) => info!(
            "proxied {} bytes from client to upstream, {} bytes from upstream to client",
            n, m
        ),
        Err(e) => info!("error is :{}", e),
    };
    Ok(())
}
fn resolve_config() -> Config {
    Config {
        listen_addr: "0.0.0.0:9001".to_string(),
        upstream_addr: "127.0.0.1:9000".to_string(),
    }
    // window 访问必须是12.0.0.1 这样的地址 不能是 0.0.0.0
}
