use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use azalea::swarm::prelude::*;

use afk_companion_bot::bot::handler::{BotState, SwarmState, handle};
use afk_companion_bot::bridge;
use afk_companion_bot::config::Config;
use afk_companion_bot::console;
use afk_companion_bot::shared::Shared;

async fn swarm_handle(_swarm: Swarm, _event: SwarmEvent, _state: SwarmState) -> anyhow::Result<()> {
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Turn on colors + UTF-8 (Windows) and print the banner before anything else,
    // so the very first thing the operator sees is a clean, structured screen.
    console::init();
    console::banner(env!("CARGO_PKG_VERSION"));

    // Azalea installs the global tracing subscriber itself (through its bevy
    // LogPlugin). We only point it at our preferred filter via RUST_LOG when the
    // user hasn't set one, so we don't fight it for the global logger.
    if std::env::var_os("RUST_LOG").is_none() {
        // SAFETY: runs at the very start of main, before any threads are spawned.
        unsafe {
            // The pathfinder logs a warning every time a path is patched, which
            // is normal on laggy servers and only clutters the console; keep it
            // at error so the output stays clean.
            std::env::set_var("RUST_LOG", "info,azalea::pathfinder=error");
        }
    }

    let config = Config::load_or_setup("config.json")?;
    let server = config.server.clone();
    let reconnect = config.reconnect_seconds;

    // Login token caches live under the data directory (default `.afk`), one
    // file per account, kept out of the Minecraft folder.
    let data_dir = config.data_path();
    std::fs::create_dir_all(&data_dir)
        .with_context(|| format!("cannot create data directory {}", data_dir.display()))?;

    // Authenticate every account first. Microsoft device-code prompts print here,
    // in a clean console, before any bridge or swarm log lines appear.
    let mut accounts = Vec::new();
    for account_config in &config.accounts {
        accounts.push(
            account_config
                .to_account(&data_dir, config.ms_client_id.as_deref())
                .await?,
        );
    }

    let shared = Arc::new(Shared::new(config.clone()));

    // The bridge never touches the bot, so a normal spawn is safe here.
    let bridge_shared = shared.clone();
    tokio::spawn(async move {
        bridge::serve(bridge_shared).await;
    });

    // Use the names Microsoft (or offline) actually gave us, not the local cache
    // keys, so the console shows who really logged in.
    let names: Vec<String> = accounts.iter().map(|a| a.username.clone()).collect();
    console::section("Connecting");
    console::info(&format!("Joining {server} as {}", names.join(", ")));
    console::hint(&format!(
        "Bot bridge on ws://127.0.0.1:{} · open Minecraft and press F7",
        config.bridge_port
    ));

    // Each account becomes its own isolated bot. They share one swarm (one
    // process, one ECS world view) but each keeps a separate `BotCtx`.
    let mut builder = SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle);
    for account in accounts {
        builder = builder.add_account_with_state(account, BotState);
    }

    // `.start().await` returns an AppExit signal, not a Result. We just drop
    // it; the program ends when the swarm ends.
    let _exit = builder
        .set_swarm_state(SwarmState { shared })
        .join_delay(Duration::from_millis(500))
        .reconnect_after(Duration::from_secs(reconnect))
        .start(server.as_str())
        .await;
    Ok(())
}
