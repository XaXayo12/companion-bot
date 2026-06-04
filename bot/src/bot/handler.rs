use std::sync::Arc;

use azalea::Event;
use azalea::ecs::prelude::*;
use azalea::player::GameProfileComponent;
use azalea::prelude::*;

use crate::bot::behavior;
use crate::shared::{BotCtx, Mode, Shared};

#[derive(Clone, Default, Component)]
pub struct BotState;

#[derive(Clone, Resource)]
pub struct SwarmState {
    pub shared: Arc<Shared>,
}

impl Default for SwarmState {
    fn default() -> Self {
        // Always overridden by set_swarm_state in main; never actually used.
        Self {
            shared: Arc::new(Shared::new(crate::config::Config {
                server: String::new(),
                data_dir: ".afk".into(),
                language: "en".into(),
                ms_client_id: None,
                owner: String::new(),
                bridge_port: 0,
                reconnect_seconds: 8,
                accounts: Vec::new(),
            })),
        }
    }
}

pub async fn handle(bot: Client, event: Event, _state: BotState) -> anyhow::Result<()> {
    let shared = bot.resource::<SwarmState>().shared.clone();

    match event {
        Event::Init => {
            // Init fires before the client has a GameProfileComponent, so we must
            // not query the username here (doing so panics). Just configure the
            // client and report that the link is connecting.
            bot.set_client_information(azalea::ClientInformation {
                view_distance: 8,
                ..Default::default()
            });
            shared.status("connecting");
        }
        other => {
            // Every other event happens once the profile is attached. Resolve the
            // per-account context safely and skip if it is somehow not ready yet.
            let Some(profile) = bot.get_component::<GameProfileComponent>() else {
                return Ok(());
            };
            let ctx = shared.ctx(&profile.name);
            match other {
                Event::Login | Event::Spawn => {
                    ctx.status("online");
                    ctx.log("In game.");
                    // Tell the mod which accounts exist so the GUI selector stays current.
                    shared.announce_roster();
                }
                Event::Chat(packet) => {
                    handle_chat(&bot, &ctx, &packet, &shared.owner());
                }
                Event::Death(_) => {
                    ctx.log("Died, respawning.");
                }
                Event::Disconnect(reason) => {
                    ctx.status("disconnected");
                    let detail = reason
                        .map(|r| r.to_string())
                        .unwrap_or_else(|| "unknown".into());
                    ctx.log(&format!("Disconnected: {detail}"));
                }
                Event::Tick => {
                    behavior::tick(&bot, &ctx);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// Whisper a private reply to a player in game (`/msg <name> ...`), if we know
/// who to answer. Servers that support `/msg` (vanilla `/tell`, Essentials) will
/// deliver it privately, so other players never see the bot's replies.
pub fn whisper(bot: &Client, target: Option<&str>, text: &str) {
    if let Some(name) = target {
        bot.chat(format!("/msg {name} {text}"));
    }
}

/// Log to the mod console and whisper the same line back to the player so they
/// get feedback in game, including the reason when something does not work.
fn reply(bot: &Client, ctx: &Arc<BotCtx>, target: Option<&str>, text: &str) {
    ctx.log(text);
    whisper(bot, target, text);
}

fn handle_chat(bot: &Client, ctx: &Arc<BotCtx>, packet: &azalea::chat::ChatPacket, owner: &str) {
    let message = packet.message().to_string();
    let sender = packet.sender();

    // Mirror chat into the mod console for this bot.
    ctx.log(&message);

    // Commands only work through a private whisper: in game you type
    // `/msg <bot-name> !follow`. Ordinary public chat is never treated as a
    // command, even if it starts with `!`.
    if !packet.is_whisper() {
        return;
    }

    // Only the owner may command the bot. The owner is your own Minecraft name,
    // which the mod reports automatically the moment you launch the game (the
    // `set_owner` command). If no owner is set — the mod isn't linked, or you run
    // the bot on its own without setting `owner` in the config — the bot obeys
    // nobody, so a stranger can never whisper it into doing something.
    match sender.as_deref() {
        Some(name) if !owner.is_empty() && name.eq_ignore_ascii_case(owner) => {}
        _ => return,
    }

    // `content()` strips the "X whispers to you:" wrapper, leaving just the
    // message (e.g. "!pull"). We require it to START with `!` so a whisper that
    // merely contains a `!` (e.g. "nice! gg") never triggers a command.
    let content = packet.content();
    let Some(rest) = content.trim().strip_prefix('!') else {
        return;
    };
    let mut parts = rest.split_whitespace();
    let Some(command) = parts.next() else {
        return;
    };
    let arg = parts.next().map(|s| s.to_string());

    // The player to answer is whoever whispered us.
    let replier = sender.clone();
    let who = arg.clone().or_else(|| sender.clone());

    // Remember the sender so async jobs (pull, mine) can whisper their result.
    ctx.rt.lock().command_sender = replier.clone();

    // Every command below mutates only this bot's runtime, so the same chat line
    // makes each connected bot act on its own state independently.
    match command.to_lowercase().as_str() {
        "follow" => {
            let mut rt = ctx.rt.lock();
            rt.mode = Mode::Follow;
            rt.follow_target = who.clone();
            drop(rt);
            match who {
                Some(name) => reply(bot, ctx, replier.as_deref(), &format!("Following {name}.")),
                None => reply(bot, ctx, replier.as_deref(), "Following you."),
            }
        }
        "protect" => {
            let mut rt = ctx.rt.lock();
            rt.mode = Mode::Protect;
            rt.protect_target = who.clone();
            drop(rt);
            reply(bot, ctx, replier.as_deref(), "Protecting you. I will fight off hostiles and stay close.");
        }
        "stop" => {
            let mut rt = ctx.rt.lock();
            rt.mode = Mode::Idle;
            rt.killaura_on = false;
            rt.mining_active = false;
            rt.last_goal = None;
            drop(rt);
            reply(bot, ctx, replier.as_deref(), "Stopped.");
        }
        "come" => {
            if let Some(name) = who {
                let mut rt = ctx.rt.lock();
                rt.mode = Mode::Follow;
                rt.follow_target = Some(name);
                drop(rt);
                reply(bot, ctx, replier.as_deref(), "Coming to you.");
            } else {
                reply(bot, ctx, replier.as_deref(), "I cannot see you. Get closer and try again.");
            }
        }
        "sel" => {
            match sender
                .as_deref()
                .and_then(|n| crate::bot::world_scan::find_player(bot, n).map(|(_, p)| p))
            {
                Some(target) => {
                    let pos = azalea::BlockPos::new(
                        target.x.floor() as i32,
                        target.y.floor() as i32,
                        target.z.floor() as i32,
                    );
                    let mut rt = ctx.rt.lock();
                    let which = if arg.as_deref() == Some("2") {
                        rt.sel2 = Some(pos);
                        2
                    } else {
                        rt.sel1 = Some(pos);
                        1
                    };
                    drop(rt);
                    reply(
                        bot,
                        ctx,
                        replier.as_deref(),
                        &format!("Point {which} set at {} {} {}.", pos.x, pos.y, pos.z),
                    );
                }
                None => reply(
                    bot,
                    ctx,
                    replier.as_deref(),
                    "I cannot see you to mark the point. Get within render distance.",
                ),
            }
        }
        "mine" => {
            let mut rt = ctx.rt.lock();
            if rt.sel1.is_some() && rt.sel2.is_some() {
                rt.mode = Mode::Mine;
                drop(rt);
                reply(bot, ctx, replier.as_deref(), "Mining the selected area.");
            } else {
                drop(rt);
                reply(
                    bot,
                    ctx,
                    replier.as_deref(),
                    "Set both corners first: !sel 1 then !sel 2.",
                );
            }
        }
        "pull" => {
            ctx.rt.lock().pull_request = Some(arg.unwrap_or_else(|| "manual".into()));
            reply(bot, ctx, replier.as_deref(), "Looking for a stasis chamber...");
        }
        "help" | "commands" => {
            reply(
                bot,
                ctx,
                replier.as_deref(),
                "Commands: !follow !protect !come !stop !sel 1|2 !mine !pull",
            );
        }
        other => {
            reply(
                bot,
                ctx,
                replier.as_deref(),
                &format!("Unknown command '{other}'. Try !help."),
            );
        }
    }
}
