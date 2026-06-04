<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Il%20tuo%20secondo%20account%20Minecraft,%20che%20gioca%20mentre%20non%20ci%20sei&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; [Français](README.fr.md) &nbsp; [Español](README.es.md) &nbsp; [Deutsch](README.de.md) &nbsp; [Русский](README.ru.md) &nbsp; [Português](README.pt.md) &nbsp; **Italiano**

</div>

## Cos'è

Un bot di secondo account per **Minecraft 1.21.11** che resta nel mondo al posto
tuo. Ti segue, allontana i mostri, mangia quando ha fame, tiene un totem nella mano
secondaria, mina un'area e può riportarti a casa con una camera di stasi. Lo
controlli dal tuo Minecraft con un piccolo menu (**F7**) o con messaggi privati.

È composto da due parti:

| Parte | Cos'è | Dove gira |
|-------|-------|-----------|
| **Il bot** (`bot/`) | Un vero client Minecraft senza finestra, in Rust su [Azalea](https://github.com/azalea-rs/azalea). Accede col tuo secondo account e fa il lavoro. | Una console sul tuo PC (Windows, macOS, Linux). |
| **La mod** (`mod/`) | Una piccola mod [Fabric](https://fabricmc.net) per il tuo Minecraft. Disegna il menu e gli avvisi e invia ordini al bot. | Dentro il tuo Minecraft normale. |

Comunicano solo dentro il tuo computer (`ws://127.0.0.1:8765`).

> **Leggi.** Usare un secondo account AFK viola le regole di molti server. Usa AFK
> Companion solo su server che permettono account alternativi. Sei responsabile dei
> tuoi account.

## 1. Scaricare i file (niente compilazione)

Dalla pagina [**Releases**](../../releases), scarica:

1. Il bot per il tuo sistema:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. La mod: `afk-companion-0.1.1.jar`

## 2. Installare la mod

1. Installa **Fabric Loader** per Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Installa **Fabric API** per 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Metti `afk-companion-0.1.1.jar` e il jar di Fabric API in `.minecraft/mods`.
4. Avvia Minecraft con il profilo Fabric.

## 3. Avviare il bot

Esegui il file del bot. Su macOS e Linux: `chmod +x afk-companion-bot-*` poi
`./afk-companion-bot-*`.

La prima volta fa alcune domande semplici (premi **Invio** per il valore tra
`[parentesi]`):

1. **Lingua:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` o `IT`.
2. **Tipo di account:** `1` per **Microsoft** (predefinito) o `2` per
   cracked/offline.
3. L'**IP** e la **porta** del server.

Non digiti mai la tua e-mail né un nome utente. Per un account Microsoft, il bot
mostra all'avvio un link e un codice: apri il link, digita il codice, conferma.
Solo una volta. (Solo un account offline chiede un nome, perché quel nome è la sua
identità.)

Scrive `config.json` accanto a sé ed entra nel server.

> **Suggerimento.** Il bot e tu dovete essere sullo stesso server. Il bot può
> seguirti, proteggerti o tirarti solo se ti vede nel mondo.

## 4. Giocare

* **F7**: apre il **menu di AFK Companion**. Clicca per attivare o disattivare. Con
  più bot, clicca una scheda account in alto per scegliere quale controllare, o
  `ALL`.
* **V**: chiede al bot di tirarti con una camera di stasi.

I comandi si danno solo via messaggio privato, quindi gli altri non li vedono, e
solo **tu** (chi ha avviato il gioco con la mod) puoi comandare il bot:

```
/msg <bot> !follow    il bot ti segue
/msg <bot> !protect   il bot ti protegge e combatte i mostri
/msg <bot> !come      il bot viene una volta
/msg <bot> !stop      ferma tutto
/msg <bot> !sel 1     segna l'angolo 1 (mettiti dove vuoi)
/msg <bot> !sel 2     segna l'angolo 2
/msg <bot> !mine      mina il box tra i due angoli
/msg <bot> !pull      elenca o attiva le camere di stasi vicine
/msg <bot> !help      mostra i comandi
```

Il bot ti risponde in privato cosa sta facendo, o il motivo esatto se non può.

## Cosa sa fare (il menu)

* **Combat:** KillAura (colpisce i mostri a portata legittima, *Stay still* per le
  farm di spawner), Protect (ti segue e difende).
* **Movement:** Follow (ti segue, distanza regolabile, evita lava e dirupi, sa
  nuotare), Hold Position, Goto Me, Sprint, Sneak.
* **Render:** HUD Info, Bot Tracer, Background.
* **Player:** Auto-Eat (sempre attivo, mangia quando il cibo scende al livello
  impostato, predefinito 17), Auto-Armor, Auto-Totem, Auto-Disconnect, Auto-Pull,
  Death Alert.
* **Misc:** Notifications (attivo di default, mostra i cambi di stato e gli avvisi
  come "I can't see you" nella tua chat), Deposit, Drop Trash, Mine.

## Il file `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "it",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server`: il server. `language`: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`.
* `owner`: lascialo vuoto, la mod invia il tuo nome automaticamente. Compilalo solo
  se usi il bot senza la mod.
* `accounts`: per Microsoft, `username` è solo un'etichetta locale per l'accesso
  salvato; per `offline` è il nome nel gioco. Aggiungi più voci per più bot
  contemporaneamente.

## Risoluzione dei problemi

* **"Waiting for the bot"**: il bot non è in esecuzione o non è raggiungibile.
  Controlla che la console del bot sia aperta e che `bridge_port` corrisponda.
* **Follow non fa nulla, o "I can't see you"**: il bot agisce solo sullo stesso
  server ed entro la sua distanza di rendering. Entra nello stesso server e
  avvicinati. Attiva il modulo **Notifications** se non vedi il messaggio.
* **Il mio comando sussurrato non ha fatto nulla**: solo chi ha avviato il gioco
  con la mod può comandare il bot, e solo via `/msg`. Assicurati di sussurrare dal
  tuo account, non nella chat pubblica.
* **Accesso Microsoft non riuscito**: apri il link esatto, digita il codice,
  conferma con l'account giusto. Elimina il file in `.afk`, o *log out* dal menu,
  per ricominciare.

## Compilare da sé

```bash
cd bot && cargo run            # bot (toolchain Rust nightly)
cd mod && ./gradlew build      # mod (JDK 21), output build/libs/afk-companion-0.1.1.jar
```

Licenza [MIT](../LICENSE).
