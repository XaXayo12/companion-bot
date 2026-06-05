# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Work on the `test` branch, pending in-game validation before a release.

### Added
- **Say bar.** A chat input at the bottom of the F7 menu: type a line and the
  selected bot says it in the in-game chat (`say` command).
- **Console account menu settings.** From the startup menu you can now change the
  server (`s`) and the language (`g`) on the fly; the menu re-localizes live and
  the choice is saved to `config.json`.
- **Water-bucket clutch (MLG).** When the bot falls far enough to get hurt and
  holds a water bucket, it aims the bucket down to land safely, then scoops the
  water back up.
- **Auto-tool.** The bot mines with the right tool for each block (pickaxe, axe,
  or shovel) and fights with its best weapon (a sword, or an axe if it hits
  harder). It keeps a shield in its off-hand when no totem is needed there.
- **Configurable Microsoft client id** (`ms_client_id` in `config.json`),
  defaulting to the Minecraft launcher's public id so the device-code sign-in
  works on accounts that reject Azalea's built-in id ("first party application").

### Changed
- **Legit reach.** The bot only mines blocks within vanilla reach (~4.5 blocks)
  and only attacks within vanilla reach (3.0 blocks), aiming on its cursor, even
  if the killaura range is set higher.
- **Human-like timing.** Attacks, chest opening and item moves, drops, and head
  movement are randomized, so the bot is not frame-perfect. The head now turns
  smoothly instead of snapping, ending the 360-degree flicks.
- **Never lose valuables.** Drop-trash and deposit now keep every tool, weapon,
  piece of armor, food, block, and rare material (netherite, ingots, gems,
  enchanted books, elytra, shulker boxes, ...). Only true junk is dropped.
- **Stasis detection.** It no longer requires soul sand under the pearl (which
  detected nothing on most builds); it now finds any held ender pearl.

## [0.1.1] - 2026-06-04

First working public release. Pre-built downloads are attached below, so you do
**not** need to compile anything. Grab the bot for your OS and the mod `.jar`,
follow the README, and you're set. The bot actually moves, follows, swims,
fights, eats when hungry, and replies in game; the in-game menu controls it
without typing anything.

### Features

- **Rust bot** (`bot/`), built on Azalea 0.15.1 for Minecraft 1.21.11. Config
  loader, shared per-bot runtime with a JSON protocol parser, local WebSocket
  bridge, per-tick priority engine (survive > combat > follow), ECS world scan,
  and async mining + stasis-pull jobs.
- **Clean, structured startup console.** A banner and colored, step-by-step
  prompts that look the same on Windows, macOS, and Linux (the bot switches the
  Windows console to UTF-8 + ANSI itself).
- **Seven-language first-run setup**: English, French, Spanish, German,
  Russian, Portuguese, Italian. The chosen language is saved and reused by the
  account menu. Microsoft sign-in (device-code link) or cracked/offline.
- **Microsoft login with per-account token cache** under a `.afk` folder
  (configurable via `data_dir`), refreshed automatically so you sign in once.
- **Account menu** on relaunch: start all, start one, add, remove, or log out.
- **Fabric mod** (`mod/`): 20 modules across 5 categories, a draggable ClickGUI
  on **F7**, an account selector to control several bots, a safety HUD (low HP /
  no totem, the selected bot's mode + distance, a target tracer), an
  auto-reconnecting WebSocket client, and a private whisper command parser
  (`/msg <bot> !follow`, also `!protect`, `!come`, `!stop`, `!sel`, `!mine`,
  `!pull`, `!help`).
- **Multi-account isolation:** several accounts log in at once, each with its own
  runtime, so commands and modes never leak between bots. The protocol carries a
  `bot` field and a `roster` event for the GUI selector.
- **Controls on both sides:** auto-totem, auto-armor, auto-disconnect, goto,
  follow distance, sprint, sneak, deposit, drop-trash, and the auto-eat level,
  each wired through the shared mod and bot protocol.
- **Mod key bindings translated** into all seven languages.

### Fixed

- **The menu's Follow and Protect did nothing.** They send no name (you *are* the
  target), but the bot had no one to follow. The bot now falls back to the owner
  the mod reports (your own Minecraft name), so Follow/Protect work straight
  from the menu.
- **Silent failure across servers.** If you press Follow/Protect while the bot is
  on a different server (or you're out of its render distance), it used to do
  nothing with no explanation. It now warns you in chat with an "I can't see you"
  message, through the Notifications module (which previously did nothing at all
  and now prints the bot's status changes and warnings).
- **The HUD showed a fake food value.** Food is now the bot's real hunger level,
  read from the game.
- **Auto-eat was a blind timer.** The bot now eats only when its real food level
  drops to the threshold you set (default 17), the way a player does, instead of
  eating on a clock whether full or not.
- Block detection for stasis chambers (soul sand) and their trapdoors now reads
  the block registry id instead of a debug string, so it stays correct.
- Startup panic `missing component GameProfileComponent`: the username is
  resolved with `get_component` only after the profile is attached.
- **The bot froze in place and spammed "pathfinder timeout / path obstructed".**
  Manual movement now stands down while a path is running, so the bot walks,
  jumps, and climbs instead of stuttering on the spot.
- **Follow / combat re-computed a full path every tick** (laggy, jerky). Paths
  are only re-issued when the target really moves.
- **The bot could not swim.** It now takes over in water (it faces the target,
  swims forward, and surfaces), so it can follow you across rivers and lakes.
- **Protect now moves**, closing the distance to the threat and to you.
- **The bot whispers back.** Every command and job replies privately in game with
  what it's doing or the exact reason it could not.
- Crash on first run when the Microsoft e-mail was left blank: the setup refuses
  an empty value and asks again.
- A `!` anywhere in chat used to trigger commands; the bot now only reacts when a
  message *starts* with `!`.

### Changed

- **Owner is auto-detected.** The mod tells the bot your Minecraft name, so only
  you can command it by whisper, with nothing to configure. The optional `owner`
  config field is only for running the bot without the mod.
- Bot and mod versions are aligned at `0.1.1`.
- The auto-eat control is now a food **level** (`eat_threshold`), not a time
  interval (`eat_interval`).
- HUD: a readable background panel, an online/offline accent stripe, and a live
  line showing the bot's latest action or reply.
- The pathfinder's routine "patching path" warnings are quieted to keep the
  console clean on laggy servers.
- Disabled Azalea's `packet-event` feature so the swarm event channel no longer
  floods, lowering CPU use.

### Build & CI

- **CI** (`.github/workflows/ci.yml`) runs `cargo check` and the Gradle build on
  every push and PR.
- **Release** (`.github/workflows/release.yml`) builds the bot for Windows,
  Linux, and macOS plus the mod `.jar`, and publishes them on the GitHub Release.

### Known issues

- Two Dependabot advisories (`hickory-proto` ≤ 0.25.2, a DNS library) are pulled
  in transitively by Azalea 0.15.1, which pins `hickory-resolver ^0.25.2`. The
  fix needs a newer Azalea, so it cannot be resolved here. The risk is CPU
  exhaustion on crafted DNS responses (a malicious DNS server), not code
  execution or data exposure.
- In-game feel (combat timing, swimming, stasis pulling) can only be fully judged
  on a live server; the build is verified to compile cleanly, pass its tests, and
  be clippy-clean.

### Notes

- Running an AFK second-account bot is against the rules on many Minecraft
  servers. Use it only where alternate accounts are allowed.
