<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Your%20Minecraft%20second%20account,%20playing%20while%20you%20are%20away&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

<img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=600&size=24&pause=1000&color=EF4444&center=true&vCenter=true&width=680&height=50&lines=Follows+you+and+fights+mobs+off+your+back;Eats%2C+totems%2C+mines%2C+pulls+you+home;One+console%2C+one+menu%2C+full+control" alt="what it does"/>

<br/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)
[![Rust](https://img.shields.io/badge/Rust-nightly-dc2626?style=for-the-badge&logo=rust&logoColor=white&labelColor=000000)](https://www.rust-lang.org/)

[English](README.md) &nbsp; [Français](docs/README.fr.md) &nbsp; [Español](docs/README.es.md) &nbsp; [Deutsch](docs/README.de.md) &nbsp; [Русский](docs/README.ru.md) &nbsp; [Português](docs/README.pt.md) &nbsp; [Italiano](docs/README.it.md)

</div>

## What it is

AFK Companion is a friendly second-account bot for **Minecraft 1.21.11**. It
stays in the world for you: it follows you, fights mobs off your back, eats when
it gets hungry, keeps a totem in its off-hand, mines an area, and can pull you
home with a stasis chamber. You drive it from inside your own Minecraft with a
small menu (press **F7**) or with private chat messages.

It comes in two pieces that work together:

| Piece | What it is | Where it runs |
|-------|-----------|---------------|
| **The bot** (`bot/`) | A real Minecraft client with no window, written in Rust on top of [Azalea](https://github.com/azalea-rs/azalea). It logs in your second account and does the work. | A console window on your PC (Windows, macOS, or Linux). |
| **The mod** (`mod/`) | A small [Fabric](https://fabricmc.net) mod for your own Minecraft. It draws the menu and the on-screen alerts and sends orders to the bot. | Inside your normal Minecraft. |

They talk to each other over your own computer (`ws://127.0.0.1:8765`). Nothing
leaves your machine except the two Minecraft accounts joining the server.

> **Please read.** Running a second AFK account is against the rules on many
> servers. Only use AFK Companion on servers that allow alternate accounts. You
> are responsible for your accounts.

## 1. Get the files (no compiling needed)

Everything is pre-built. Open the [**Releases**](../../releases) page and download:

1. The bot for your system:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. The mod: `afk-companion-0.1.1.jar`

Prefer to build it yourself? See [Build from source](#build-from-source).

## 2. Install the mod

The mod needs Fabric, just like most mods:

1. Install **Fabric Loader** for Minecraft **1.21.11** (from <https://fabricmc.net/use>).
2. Install **Fabric API** for 1.21.11 (from <https://modrinth.com/mod/fabric-api>).
3. Put `afk-companion-0.1.1.jar` and the Fabric API jar into your
   `.minecraft/mods` folder.
4. Start Minecraft with the Fabric profile.

That is it. You will set it up in step 4.

## 3. Start the bot

Run the bot file you downloaded:

* **Windows:** double-click the `.exe`, or run it from a terminal.
* **macOS and Linux:** make it runnable once, then run it:
  ```bash
  chmod +x afk-companion-bot-*
  ./afk-companion-bot-*
  ```

The first time it runs, it asks a few simple questions in a clean, colored
console. Press **Enter** to accept the value shown in `[brackets]`:

1. **Language:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT`, or `IT`.
2. **Account type:** `1` for **Microsoft** (a normal paid account, the default)
   or `2` for cracked/offline.
3. **Server IP** and **port** to join.

You never type your e-mail or a username. For a Microsoft account, the bot prints
a link and a code at startup: open the link, type the code, and approve. You do
this once, and the login is remembered. (Only an offline account asks for a name,
because that name is its identity.)

It saves your answers in `config.json` next to the bot, then joins the server. On
later runs it shows a small menu so you can start, add, remove, or sign out an
account.

> **Tip.** The bot and you should join the same server. The bot can only follow,
> protect, or pull you if it can actually see you in the world.

## 4. Play

While in game on the same server:

* Press **F7** to open the **AFK Companion menu**. Click an option to turn it on
  or off. If you run several bot accounts, click an account chip at the top to
  choose which one the buttons control, or `ALL`.
* Press **V** at any time to ask the bot to pull you with a stasis chamber.

Commands are given by **private message** only, so other players never see them,
and only **you** (the player who launched the game with the mod) can command the
bot:

```
/msg <bot-name> !follow      the bot follows you
/msg <bot-name> !protect     the bot guards you and fights mobs
/msg <bot-name> !come        the bot walks to you once
/msg <bot-name> !stop        stop everything
/msg <bot-name> !sel 1       mark mining corner 1 (stand where you want it)
/msg <bot-name> !sel 2       mark mining corner 2
/msg <bot-name> !mine        mine the box between the two corners
/msg <bot-name> !pull        list or activate nearby stasis chambers
/msg <bot-name> !help        show the command list
```

The bot whispers back what it is doing, or the exact reason it cannot, so you are
never left guessing.

## What it can do (the menu)

The menu groups everything into five tabs. Toggle what you want.

**Combat**
* **KillAura:** attacks hostile mobs in legit reach, great for mob farms. The
  *Stay still* option keeps it in place for spawner farming.
* **Protect:** follows you and fights off anything that attacks.

**Movement**
* **Follow:** follows you, with an adjustable distance. It will not walk into lava
  or off cliffs, and it can swim across water to keep up.
* **Hold Position:** stand still right here.
* **Goto Me:** walk to where you are standing once.
* **Sprint and Sneak:** move sprinting or sneaking.

**Render**
* **HUD Info:** bot status, health, food, mode, and distance, top-left.
* **Bot Tracer:** a small marker pointing to where your bot is.
* **Background:** dim the screen behind the menu.

**Player**
* **Auto-Eat** (always on): the bot eats by itself when its food drops to the
  level you set, default 17.
* **Auto-Armor:** equips the best armor it is carrying.
* **Auto-Totem:** keeps a Totem of Undying in the off-hand.
* **Auto-Disconnect:** leaves the server if its health drops too low.
* **Auto-Pull:** asks the bot to pull you with a stasis chamber when your health
  gets low.
* **Death Alert:** big on-screen warning when you are in danger or have no totem.

**Misc**
* **Notifications** (on by default): prints the bot's status changes and warnings
  (like "I can't see you") into your chat.
* **Deposit:** put junk items into the nearest chest.
* **Drop Trash:** drop junk items on the ground.
* **Mine:** mine the area you marked with `!sel 1` and `!sel 2`.

## The `config.json` file

The bot writes this next to itself on first run. You can edit it by hand later:

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "en",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

| Field | Meaning |
|-------|---------|
| `server` | The server to join, for example `play.example.com` or `1.2.3.4:25566`. |
| `data_dir` | Folder for the saved logins, one small file per account. |
| `language` | Console language: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`. |
| `owner` | Usually left empty. The mod tells the bot your name automatically, so only you can command it. Set it by hand only if you run the bot without the mod. |
| `bridge_port` | The local port the mod connects to, default `8765`. |
| `reconnect_seconds` | How long to wait before reconnecting after a drop. |
| `accounts` | One or more accounts. For Microsoft, `username` is just a local label for the saved login; for `offline`/`cracked` it is the in-game name. |

To run several bots at once, add more entries to `accounts`. Each one gets its
own state, and the menu's account chips let you control them separately or all
together.

## Troubleshooting

**The menu says "Waiting for the bot".**
The bot is not running, or is not reachable. Make sure the bot console is open and
that `bridge_port` in `config.json` matches the mod, default `8765`.

**I press Follow and nothing happens, or chat says "I can't see you".**
The bot can only follow, protect, or pull you when it is on the same server and
you are within its render distance. Join the same server, get closer, and try
again. If you do not see the message, turn on the **Notifications** module.

**My whispered command did nothing.**
Only the player who launched the game with the mod can command the bot, and only
through `/msg`. Make sure you are whispering from your own account, not typing in
public chat.

**The Microsoft sign-in did not work.**
Open the exact link the bot printed, type the code, and approve the request with
the right Microsoft account. Delete the matching file in the `.afk` folder, or use
the menu's *log out*, to sign in again from scratch.

**On a few servers the console prints packet parse errors.**
Some servers send packets Azalea 0.15.1 does not fully parse yet. If the bot keeps
running, these are harmless and safe to ignore.

## Build from source

You only need this if you want to change the code. Otherwise use the Releases.

```bash
# Bot, needs the Rust nightly toolchain (pinned in bot/rust-toolchain.toml)
cd bot
cargo run                # build and run against ./config.json
cargo build --release    # optimized binary in target/release/
cargo test               # unit and protocol tests

# Mod, needs JDK 21
cd mod
./gradlew build          # jar in build/libs/afk-companion-0.1.1.jar
./gradlew runClient      # launch Minecraft with the mod for testing
```

More detail for contributors is in [`DEVELOPMENT.md`](DEVELOPMENT.md).

## License

[MIT](LICENSE). Built on [Azalea](https://github.com/azalea-rs/azalea) and
[Fabric](https://fabricmc.net).

<div align="center">
<img src="https://capsule-render.vercel.app/api?type=waving&color=0:dc2626,100:000000&height=120&section=footer" width="100%" alt="footer"/>
</div>
