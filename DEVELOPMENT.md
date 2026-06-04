# Development

Internal notes for contributors. User-facing docs live in [`README.md`](README.md);
this file is about the code.

## What this project is

Two programs that run on the same PC and talk to each other:

- `bot/`, Rust, built on Azalea 0.15.1 (Minecraft 1.21.11). A headless
  second-account client that joins the server, survives, fights, follows, mines,
  and pulls the player via stasis chambers. Driven by private in-game whispers (`/msg <bot> !command`)
  and a local WebSocket from the mod.
- `mod/`, Fabric client mod (Minecraft 1.21.11). A ClickGUI (F7) plus a
  safety HUD. The mod does not control your own character, it sends orders to the
  bot and shows you alerts (low HP, no totem). It auto-connects to the bot over
  `ws://127.0.0.1:8765`.

## Architecture

```
┌──────────────────────┐    WebSocket on 127.0.0.1:8765    ┌──────────────────────┐
│  Minecraft + Mod     │◄────────────────────────────────►│  Bot (Rust)          │
│  (the player)        │   {"cmd":"follow"} → JSON cmds    │  (2nd account)       │
│  - reads YOUR HP     │   {"event":"telemetry"} ← events  │  - real client       │
│  - draws the GUI     │                                   │  - survives/fights   │
│  - shows alerts      │                                   │  - mines/pulls       │
└──────────┬───────────┘                                   └──────────┬───────────┘
           │                                                          │
           ▼                                                          ▼
   ┌─────────────────────── Minecraft Server 1.21.11 ────────────────────┐
   └──────────────────────────────────────────────────────────────────────┘
```

The mod is the control surface, the bot is the worker. They both connect to the
same Minecraft server independently and communicate only over the local
WebSocket. The bot still works without the mod (whisper `/msg <bot> !command`); the mod still
works without the bot (the HUD alerts still render and bot commands are queued).

## The protocol (mod ↔ bot)

JSON over a single WebSocket on `127.0.0.1:8765`. The authoritative definitions
are `bot/src/shared.rs` (`Shared::apply_command` routes by id, `BotCtx::apply`
parses) and `mod/src/main/java/com/afkcompanion/net/Protocol.java` (the
builders). If you change one, change the other in the same commit.

Every command carries an optional `bot` field that selects the target account:
a concrete id targets one bot, `"all"`/`"*"` (or an absent field) broadcasts to
all. `BotLink.send` injects the GUI's selected account automatically, so the
builders themselves never set it. Every event carries the `bot` id it came from.

### Mod → Bot (commands)

| `cmd` | Fields | Meaning |
|-------|--------|---------|
| `follow` | `target?: string` | Follow target player (or sender) |
| `protect` | `target?: string` | Defend target from mobs |
| `stop` |, | Stop all activities |
| `killaura` | `on: bool, range: f64, stay_still: bool` | Stay-and-kill mob farming |
| `hold_position` | `on: bool` | Stop pathfinding, stand still |
| `mine_start` |, | Mine the `!sel 1`/`!sel 2` box |
| `pull` | `arg: "manual"\|"health_low"\|"<id>"\|"<player>"` | Stasis pull |
| `auto_totem` | `on: bool` | Keep a Totem of Undying in the offhand |
| `auto_armor` | `on: bool` | Equip the best available armor |
| `auto_disconnect` | `on: bool, health: f64` | Leave the server at/below a health threshold |
| `goto` | `x: f64, y: f64, z: f64` | Pathfind to a block position |
| `follow_distance` | `distance: f64` | Follow radius (clamped 1–10) |
| `sprint` | `on: bool` | Sprint while moving |
| `sneak` | `on: bool` | Sneak |
| `deposit` |, | Deposit items into the nearest chest |
| `drop_trash` |, | Drop trash items |
| `eat_threshold` | `food: u32` | Auto-eat when the live food level is at/below this (clamped 1–19) |
| `set_owner` | `owner: string` | Global (not per-bot): the in-game name allowed to command by whisper. The mod sends your own name automatically; empty means **nobody** may command by whisper. |

### Bot → Mod (events)

| `event` | Fields |
|---------|--------|
| `status` | `bot?: string, status: string` (connecting/online/disconnected/...); a `status` with no `bot` is the bridge link itself |
| `telemetry` | `bot: string, health: f32, food: u32, x: f64, y: f64, z: f64, mode: string` (~every 10 ticks; `food` is the live Hunger value) |
| `log` | `bot?: string, text: string` (mirror of in-game chat + bot decisions) |
| `error` | `bot?: string, text: string` (a problem the player should see, e.g. "I can't see you"; the mod's Notifications module prints it to chat) |
| `roster` | `bots: string[]` (the connected account ids, for the GUI selector) |

## Commands, owner, and login

- **Whisper-only commands.** `handler.rs` only treats a chat message as a command
  when `packet.is_whisper()` is true, so public chat that merely starts with `!`
  is ignored. The message must then start with `!` (matching Azalea's testbot).
- **Owner-only.** Only the owner may command the bot by whisper. The owner is set
  live by the mod (`set_owner`, your own name from `client.getSession()`), with
  `config.owner` as the initial value. An empty owner means **nobody** can whisper
  commands, so a stranger can never drive the bot.
- **No e-mail or username at setup.** First-run setup asks language, account type,
  and server only. A Microsoft account is identified by signing in to the
  device-code link at startup, so `config.rs` only keeps a local cache key
  (`account-1`, ...). Only an offline account asks for an in-game name, since that
  name *is* the identity in offline mode.
- **Owner fallback for the GUI.** `Follow`/`Protect` from the menu send no target,
  so `behavior.rs` falls back to the owner. If the target player isn't visible
  (you're on another server or out of render distance), the bot emits an `error`
  event instead of standing there silently.

## File map

```
afk-companion/
├── README.md            ← user docs (English)
├── DEVELOPMENT.md       ← this file
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE              ← MIT
├── .github/
│   ├── workflows/ci.yml ← runs cargo check and gradle build on PRs
│   └── ISSUE_TEMPLATE/  ← bug report / feature request
├── bot/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── rust-toolchain.toml   ← nightly pin
│   ├── config.example.json
│   └── src/
│       ├── main.rs           ← swarm + bridge entrypoint, banner
│       ├── console.rs        ← cross-platform colored console (UTF-8 + ANSI on Windows)
│       ├── config.rs         ← JSON config load/create + 7-language setup
│       ├── shared.rs         ← runtime state + protocol parser
│       ├── bridge.rs         ← local WebSocket server (tokio-tungstenite)
│       └── bot/
│           ├── mod.rs        ← module index + distance helper
│           ├── handler.rs    ← Azalea per-bot event loop + chat commands
│           ├── behavior.rs   ← per-tick priority engine (survive > combat > move)
│           ├── identity.rs   ← food/sword/pickaxe/hostile tables
│           ├── world_scan.rs ← ECS queries, LoS raycast, stasis detection
│           └── tasks.rs      ← async mining + stasis pull jobs
└── mod/
    ├── gradle.properties
    ├── build.gradle
    ├── settings.gradle
    ├── gradlew / gradlew.bat
    ├── gradle/wrapper/
    └── src/main/
        ├── java/com/afkcompanion/
        │   ├── AfkCompanion.java          ← constants + SLF4J logger
        │   ├── AfkCompanionClient.java    ← ClientModInitializer; wires everything
        │   ├── config/ModConfig.java      ← JSON config in .minecraft/config/
        │   ├── module/                    ← Category, Module, Setting, ModuleManager
        │   │   └── modules/               ← 20 modules: KillAura, Protect, Follow, ...
        │   ├── gui/
        │   │   ├── Theme.java             ← accent palette
        │   │   ├── RenderUtil.java        ← rect/text drawing helpers
        │   │   ├── CategoryPanel.java     ← one draggable column (per category)
        │   │   ├── ClickGuiScreen.java    ← the menu (F7)
        │   │   └── Hud.java               ← bot status + DANGER / NO TOTEM alerts
        │   ├── net/
        │   │   ├── Protocol.java          ← JSON command builders (match shared.rs)
        │   │   ├── BotState.java          ← last-known bot state (volatile fields)
        │   │   └── BotLink.java           ← JDK WebSocket client with auto-reconnect
        │   ├── input/
        │   │   ├── Keybinds.java          ← F7 = GUI, V = pull
        │   │   └── KeybindHandler.java    ← edge-detected per-module key polling
        │   └── sensor/DangerSensor.java   ← reads YOUR HP, totem count, danger state
        └── resources/
            ├── fabric.mod.json
            └── assets/afkcompanion/lang/   ← en_us, fr_fr, es_es, de_de, ru_ru, pt_pt, it_it
```

## Build

```bash
# Bot, needs the Rust nightly toolchain (see bot/rust-toolchain.toml)
cd bot
cargo build              # debug build
cargo build --release    # release artifact
cargo run                # runs against config.json in the same folder
cargo test               # unit tests (protocol parser)

# Mod, needs JDK 21
cd mod
./gradlew build          # output: build/libs/afk-companion-0.1.1.jar
./gradlew runClient      # launches Minecraft with the mod for dev testing
```

The mod's `gradle.properties` pins the Minecraft, Yarn, Fabric Loader, Loom, and
Fabric API versions. Building the mod fetches these from the Fabric and Mojang
Maven repositories, so the build machine needs network access to them.

## Style

- Code, comments, logs, and in-game messages are in English.
- Comments are sparse, names carry the meaning. A comment explains *why*, not
  *what*.
- The bot never uses `unwrap`/`expect` outside `main.rs`. Every fallible path
  returns `Result` or logs and skips.
- The mod never crashes the game. Every event handler is wrapped in
  `try { ... } catch (Throwable ignored) {}`.
- Module names match the `byName(...)` lookups. If you rename a module, grep for
  its name first.
- Adding a `cmd` means editing both `Protocol.java` and
  `shared.rs::apply_command` in the same change.

## Useful links

- Azalea docs: https://docs.rs/azalea/0.15.1
- Fabric: https://fabricmc.net/develop
- Reference stasis bot in Azalea: https://github.com/EnderKill98/stasis-bot
