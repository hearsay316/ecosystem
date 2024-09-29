use dashmap::DashMap;
use futures::stream::SplitStream;
use futures::{SinkExt, StreamExt};
use std::fmt::Formatter;
use std::sync::Arc;
use std::{fmt, net::SocketAddr};
use tokio::sync::mpsc::Sender;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer as _;

const MAX_MESSAGE_SIZE: usize = 1024;
#[derive(Debug, Default)]
struct State {
    peers: DashMap<SocketAddr, Sender<Arc<Message>>>,
}

#[derive(Debug)]
enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}
#[derive(Debug)]
struct Peer {
    username: String,
    stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建一个控制台订阅者
    let console_layer = console_subscriber::spawn();
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry()
        .with(console_layer)
        .with(layer)
        .init();

    let addr = "127.0.0.1:8083";
    let listener = TcpListener::bind(&addr).await?;
    info!("Stating chat server on {}", addr);
    let state = Arc::new(State::default());
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from :{}", addr);
        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(state_clone, addr, stream).await {
                warn!("Failed to handle client {} :{}", addr, e);
            }
        });
    }
}
async fn handle_client(
    state: Arc<State>,
    addr: SocketAddr,
    stream: TcpStream,
) -> anyhow::Result<()> {
    let mut stream = Framed::new(stream, LinesCodec::new());
    stream.send("Enter your username :").await?;

    let username = match stream.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };
    // 通知所有人
    let mut peer = state.add(addr, username.clone(), stream).await;
    let message = Arc::new(Message::user_joined(&username.clone()));
    info!("{}", message);
    state.broadcast(addr, message).await;

    while let Some(line) = peer.stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Failed to read line from {} :{}", addr, e);
                break;
            }
        };

        let message = Arc::new(Message::chat(&peer.username.clone(), &line));
        info!("{}", message);
        state.broadcast(addr, message).await;
    }
    state.peers.remove(&addr);
    let message = Arc::new(Message::user_left(&peer.username));
    info!("{}", message);
    state.broadcast(addr, message).await;
    Ok(())
}
impl State {
    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }
            if let Err(e) = peer.value().send(message.clone()).await {
                warn!("Falied to send  message to :{} :{}", peer.key(), e);
                self.peers.remove(peer.key());
            };
        }
    }

    async fn add(
        &self,
        addr: SocketAddr,
        username: String,
        stream: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MESSAGE_SIZE);

        self.peers.insert(addr, tx);

        let (mut stream_sender, stream_receiver) = stream.split();
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = stream_sender.send(message.to_string()).await {
                    warn!("Failed to send message to :{} {}", addr, e);
                    break;
                }
            }
        });
        Peer {
            username,
            stream: stream_receiver,
        }
    }
}
// impl Default for State {
//     fn default() -> Self {
//         Self {
//             peers: DashMap::new(),
//         }
//     }
// }
impl Message {
    fn user_joined(username: &str) -> Self {
        let content = format!("{} has joined the chat", username);
        Self::UserJoined(content)
    }
    fn user_left(username: &str) -> Self {
        let content = format!("{} has left the chat", username);
        Self::UserLeft(content)
    }
    fn chat(sender: &str, content: &str) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}
impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserJoined(content) => write!(f, "[{}]", content),
            Self::UserLeft(content) => write!(f, "[{}:(]", content),
            Self::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}
