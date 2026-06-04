//! Real integration test for the WebSocket bridge and command protocol.
//!
//! This starts the actual `bridge::serve` on an ephemeral TCP port, connects a
//! real WebSocket client with tokio-tungstenite, sends command JSON over the
//! wire (the same path the Fabric mod uses), and asserts that:
//!   * commands mutate the shared `Runtime` state, and
//!   * server-side `emit`/`status`/`log`/`telemetry` produce valid JSON that
//!     arrives at the client.

use std::sync::Arc;
use std::time::Duration;

use afk_companion_bot::bridge;
use afk_companion_bot::config::Config;
use afk_companion_bot::shared::{Mode, Shared};

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

fn test_config(port: u16) -> Config {
    Config {
        server: String::new(),
        data_dir: ".afk".into(),
        language: "en".into(),
        owner: String::new(),
        bridge_port: port,
        reconnect_seconds: 8,
        accounts: Vec::new(),
    }
}

// Ask the OS for a free TCP port, then release it so the bridge can rebind it.
async fn free_port() -> u16 {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral");
    let port = listener.local_addr().expect("local_addr").port();
    drop(listener);
    port
}

async fn read_json(
    reader: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
) -> serde_json::Value {
    loop {
        let msg = tokio::time::timeout(Duration::from_secs(5), reader.next())
            .await
            .expect("timed out waiting for ws message")
            .expect("stream ended")
            .expect("ws error");
        if let Message::Text(text) = msg {
            return serde_json::from_str(text.as_str()).expect("server sent invalid JSON");
        }
        // ignore pings/pongs/binary
    }
}

#[tokio::test]
async fn websocket_round_trip_drives_protocol() {
    let port = free_port().await;
    let shared = Arc::new(Shared::new(test_config(port)));

    // Register one bot context up front; commands targeting it must route here.
    let ctx = shared.ctx("tester");

    // Subscribe to the broadcast channel directly to prove emit() works at the
    // channel level as well as over the socket.
    let mut direct_rx = shared.tx.subscribe();

    // Start the real bridge server.
    let server_shared = shared.clone();
    tokio::spawn(async move {
        bridge::serve(server_shared).await;
    });

    // Give the listener a moment to bind.
    let url = format!("ws://127.0.0.1:{port}");
    let mut stream = None;
    for _ in 0..50 {
        match tokio_tungstenite::connect_async(&url).await {
            Ok((ws, _)) => {
                stream = Some(ws);
                break;
            }
            Err(_) => tokio::time::sleep(Duration::from_millis(50)).await,
        }
    }
    let ws = stream.expect("could not connect to bridge");
    let (mut writer, mut reader) = ws.split();

    // On connect the bridge emits a status("connected") event. It should arrive
    // both on the socket and on the direct channel subscription.
    let first = read_json(&mut reader).await;
    assert_eq!(first["event"], "status");
    assert_eq!(first["status"], "connected");

    // Drain the same event off the direct channel (proves emit -> tx works).
    let direct: serde_json::Value = {
        let raw = tokio::time::timeout(Duration::from_secs(5), direct_rx.recv())
            .await
            .expect("timed out on direct channel")
            .expect("direct channel closed");
        serde_json::from_str(&raw).expect("invalid JSON on channel")
    };
    assert_eq!(direct["event"], "status");

    // --- Send several real commands over the wire and assert state changes. ---
    writer
        .send(Message::text(r#"{"cmd":"follow","bot":"tester","target":"Steve"}"#))
        .await
        .expect("send follow");
    writer
        .send(Message::text(r#"{"cmd":"sprint","bot":"tester","on":true}"#))
        .await
        .expect("send sprint");
    writer
        .send(Message::text(r#"{"cmd":"eat_threshold","bot":"tester","food":15}"#))
        .await
        .expect("send eat_threshold");
    writer
        .send(Message::text(r#"{"cmd":"goto","bot":"tester","x":10.9,"y":64.0,"z":-3.7}"#))
        .await
        .expect("send goto");

    // Wait until the commands have been applied (the server applies them on its
    // own task), polling the bot's isolated state.
    let mut applied = false;
    for _ in 0..100 {
        {
            let rt = ctx.rt.lock();
            if rt.follow_target.as_deref() == Some("Steve")
                && rt.sprinting
                && rt.eat_threshold == 15
                && rt.mode == Mode::Goto
            {
                applied = true;
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(applied, "commands were not applied to the bot's state");

    {
        let rt = ctx.rt.lock();
        assert_eq!(rt.mode, Mode::Goto);
        assert_eq!(
            rt.goto_target,
            Some(azalea::BlockPos::new(10, 64, -4)),
            "goto coords should be floored"
        );
    }

    // --- Server-originated events should reach the client as valid JSON. ---
    shared.log("hello from server");
    ctx.telemetry(20.0, 18, 1.0, 2.0, 3.0);

    // Collect a handful of events and verify each is valid JSON with an "event".
    let mut saw_log = false;
    let mut saw_telemetry = false;
    for _ in 0..10 {
        let v = read_json(&mut reader).await;
        assert!(v.get("event").is_some(), "every message must have an event");
        match v["event"].as_str() {
            Some("log") => {
                assert_eq!(v["text"], "hello from server");
                saw_log = true;
            }
            Some("telemetry") => {
                assert_eq!(v["health"], 20.0);
                assert_eq!(v["food"], 18);
                // telemetry must carry the lowercase mode string we set above.
                assert_eq!(v["mode"], "goto");
                saw_telemetry = true;
            }
            _ => {}
        }
        if saw_log && saw_telemetry {
            break;
        }
    }
    assert!(saw_log, "did not receive log event");
    assert!(saw_telemetry, "did not receive telemetry event");
}
