use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

use crate::shared::Shared;

pub async fn serve(shared: Arc<Shared>) {
    let addr = format!("127.0.0.1:{}", shared.config.bridge_port);
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::error!("bridge cannot bind {addr}: {error}");
            return;
        }
    };
    tracing::info!("bridge listening on ws://{addr}");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let shared = shared.clone();
                tokio::spawn(async move {
                    handle_client(stream, shared).await;
                });
            }
            Err(error) => tracing::warn!("bridge accept error: {error}"),
        }
    }
}

async fn handle_client(stream: TcpStream, shared: Arc<Shared>) {
    let ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return,
    };
    let (mut writer, mut reader) = ws.split();
    let mut events = shared.tx.subscribe();

    shared.status("connected");

    loop {
        tokio::select! {
            incoming = reader.next() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => shared.apply_command(text.as_str()),
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    _ => {}
                }
            }
            outgoing = events.recv() => {
                if let Ok(json) = outgoing
                    && writer.send(Message::text(json)).await.is_err() {
                        break;
                    }
            }
        }
    }
}
