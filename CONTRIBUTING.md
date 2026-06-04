# Contributing to AFK Companion

Thanks for opening this file, that already puts you ahead of most. This is a small project; the rules are short.

## Before you start

Read [`DEVELOPMENT.md`](DEVELOPMENT.md). It covers the architecture, the mod ↔ bot protocol, the build commands, and the code style. **The README is for users; DEVELOPMENT.md is for contributors.**

## Quick setup

You need:
- Rust nightly (for the bot), `rustup toolchain install nightly`
- JDK 21 (for the mod)
- A local Minecraft 1.21.11 server to test against (Paper, Spigot, or vanilla, offline mode is fine for dev)

```bash
git clone <your fork>
cd afk-companion

# Bot
cd bot && cargo build && cd ..

# Mod
cd mod && ./gradlew build && cd ..
```

## Workflow

1. **Open an issue first** if it's a new feature or a non-trivial change. A two-line description is fine. This avoids two people redoing the same work.
2. **Branch from `main`**. Branch name: `fix/<short>`, `feat/<short>`, or `docs/<short>`.
3. **One logical change per PR.** Don't bundle a refactor with a feature.
4. **Update `CHANGELOG.md`** in the `[Unreleased]` section.
5. **Run the local checks** before pushing: `cargo check` in `bot/`, `./gradlew build` in `mod/`.
6. **Describe what you tested** in the PR: whether you only ran the build, or also ran it in-game.

## Code style

- **Code, comments, logs, in-game text: English.** User-facing docs (`README.md`) are multilingual.
- **Comments are sparse.** Names should carry the meaning. A comment is for *why*, not *what*.
- **No `unwrap`/`expect`** in the bot outside of `main.rs`. Use `Result` or log-and-skip.
- **Never crash the game** from the mod. Every event handler is `try { ... } catch (Throwable ignored) {}`.

## The protocol is sacred

The JSON commands between the mod and the bot live in two files:
- `bot/src/shared.rs::apply_command` (parser)
- `mod/src/main/java/com/afkcompanion/net/Protocol.java` (builders)

**If you change one, change the other in the same PR.** Update the protocol table in `DEVELOPMENT.md`.

## Reporting bugs

Use the **Bug report** issue template. Always include:
- OS + Minecraft version
- Bot version (`cargo run` prints it) or mod jar filename
- Server type (vanilla, Paper, modded) + offline/online
- What you typed/clicked, what you expected, what happened
- Bot console output (`cargo run` logs) and mod log (`logs/latest.log`) if relevant

## Asking for help

GitHub Discussions is the right place. Don't @ maintainers, be patient.

## Security

If you find something that lets someone abuse the bot's owner system, the local WebSocket, or the auth tokens, **do not open a public issue**. Email or DM the maintainers privately.
