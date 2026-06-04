<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Dein%20zweiter%20Minecraft-Account,%20der%20spielt%20wenn%20du%20weg%20bist&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; [Français](README.fr.md) &nbsp; [Español](README.es.md) &nbsp; **Deutsch** &nbsp; [Русский](README.ru.md) &nbsp; [Português](README.pt.md) &nbsp; [Italiano](README.it.md)

</div>

## Was es ist

Ein Zweitaccount-Bot für **Minecraft 1.21.11**, der für dich in der Welt bleibt.
Er folgt dir, hält Monster fern, isst wenn er hungrig wird, behält ein Totem in
der Nebenhand, baut einen Bereich ab und kann dich mit einer Stasis-Kammer
zurückholen. Du steuerst ihn aus deinem eigenen Minecraft mit einem kleinen Menü
(**F7**) oder mit privaten Nachrichten.

Er besteht aus zwei Teilen:

| Teil | Was es ist | Wo es läuft |
|------|------------|-------------|
| **Der Bot** (`bot/`) | Ein echter Minecraft-Client ohne Fenster, in Rust mit [Azalea](https://github.com/azalea-rs/azalea). Er meldet deinen Zweitaccount an und erledigt die Arbeit. | Eine Konsole auf deinem PC (Windows, macOS, Linux). |
| **Der Mod** (`mod/`) | Ein kleiner [Fabric](https://fabricmc.net)-Mod für dein Minecraft. Er zeichnet das Menü und die Warnungen und sendet Befehle an den Bot. | In deinem normalen Minecraft. |

Sie kommunizieren nur auf deinem Computer (`ws://127.0.0.1:8765`).

> **Bitte lesen.** Ein zweiter AFK-Account verstößt auf vielen Servern gegen die
> Regeln. Nutze AFK Companion nur auf Servern, die Zweitaccounts erlauben. Du bist
> für deine Accounts verantwortlich.

## 1. Dateien holen (kein Kompilieren)

Lade auf der [**Releases**](../../releases)-Seite herunter:

1. Den Bot für dein System:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. Den Mod: `afk-companion-0.1.1.jar`

## 2. Den Mod installieren

1. Installiere **Fabric Loader** für Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Installiere **Fabric API** für 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Lege `afk-companion-0.1.1.jar` und das Fabric-API-Jar in `.minecraft/mods`.
4. Starte Minecraft mit dem Fabric-Profil.

## 3. Den Bot starten

Starte die Bot-Datei. Auf macOS und Linux: `chmod +x afk-companion-bot-*`, dann
`./afk-companion-bot-*`.

Beim ersten Start stellt er ein paar einfache Fragen (drücke **Enter** für den
Wert in `[Klammern]`):

1. **Sprache:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` oder `IT`.
2. **Kontotyp:** `1` für **Microsoft** (Standard) oder `2` für Cracked/Offline.
3. **IP** und **Port** des Servers.

Du tippst nie deine E-Mail oder einen Benutzernamen. Bei einem Microsoft-Konto
zeigt der Bot beim Start einen Link und einen Code: öffne den Link, gib den Code
ein, bestätige. Nur einmal. (Nur ein Offline-Konto fragt nach einem Namen, weil
dieser Name seine Identität ist.)

Er schreibt `config.json` daneben und betritt den Server.

> **Tipp.** Bot und du solltet auf demselben Server sein. Der Bot kann dir nur
> folgen, dich schützen oder dich ziehen, wenn er dich in der Welt sieht.

## 4. Spielen

* **F7**: öffnet das **AFK-Companion-Menü**. Klicke zum Ein- oder Ausschalten. Bei
  mehreren Bots oben einen Konto-Chip anklicken, um zu wählen, welcher gesteuert
  wird, oder `ALL`.
* **V**: bittet den Bot, dich mit einer Stasis-Kammer zu ziehen.

Befehle werden nur per privater Nachricht gegeben, andere Spieler sehen sie also
nicht, und nur **du** (wer das Spiel mit dem Mod gestartet hat) kann den Bot
steuern:

```
/msg <bot> !follow    der Bot folgt dir
/msg <bot> !protect   der Bot beschützt dich und bekämpft Monster
/msg <bot> !come      der Bot kommt einmal zu dir
/msg <bot> !stop      alles stoppen
/msg <bot> !sel 1     Ecke 1 markieren (stell dich hin)
/msg <bot> !sel 2     Ecke 2 markieren
/msg <bot> !mine      den Bereich zwischen den Ecken abbauen
/msg <bot> !pull      nahe Stasis-Kammern auflisten oder aktivieren
/msg <bot> !help      Befehle anzeigen
```

Der Bot antwortet dir privat, was er tut, oder den genauen Grund, wenn nicht.

## Was er kann (das Menü)

* **Combat:** KillAura (schlägt Monster in legitimer Reichweite, *Stay still* für
  Spawner-Farmen), Protect (folgt und verteidigt dich).
* **Movement:** Follow (folgt dir, einstellbarer Abstand, meidet Lava und
  Abgründe, kann schwimmen), Hold Position, Goto Me, Sprint, Sneak.
* **Render:** HUD Info, Bot Tracer, Background.
* **Player:** Auto-Eat (immer an, isst wenn die Nahrung auf den eingestellten Wert
  fällt, Standard 17), Auto-Armor, Auto-Totem, Auto-Disconnect, Auto-Pull, Death
  Alert.
* **Misc:** Notifications (standardmäßig an, zeigt Statusänderungen und Warnungen
  wie "I can't see you" in deinem Chat), Deposit, Drop Trash, Mine.

## Die Datei `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "de",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server`: der Server. `language`: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`.
* `owner`: leer lassen, der Mod sendet deinen Namen automatisch. Nur ausfüllen,
  wenn du den Bot ohne den Mod nutzt.
* `accounts`: bei Microsoft ist `username` nur ein lokales Label für die
  gespeicherte Anmeldung; bei `offline` ist es der Name im Spiel. Mehrere Einträge
  für mehrere Bots gleichzeitig.

## Fehlerbehebung

* **"Waiting for the bot"**: Der Bot läuft nicht oder ist nicht erreichbar. Prüfe,
  ob die Bot-Konsole offen ist und `bridge_port` übereinstimmt.
* **Follow tut nichts, oder "I can't see you"**: Der Bot wirkt nur auf demselben
  Server und innerhalb seiner Sichtweite. Geh auf denselben Server und komm näher.
  Aktiviere das Modul **Notifications**, falls du die Meldung nicht siehst.
* **Mein geflüsterter Befehl tat nichts**: Nur wer das Spiel mit dem Mod gestartet
  hat, kann den Bot steuern, und nur per `/msg`. Stelle sicher, dass du von deinem
  Konto flüsterst, nicht im öffentlichen Chat.
* **Microsoft-Anmeldung fehlgeschlagen**: Öffne den genauen Link, gib den Code
  ein, bestätige mit dem richtigen Konto. Lösche die Datei in `.afk`, oder *log
  out* im Menü, um neu zu beginnen.

## Selbst bauen

```bash
cd bot && cargo run            # Bot (Rust-Nightly-Toolchain)
cd mod && ./gradlew build      # Mod (JDK 21), Ausgabe build/libs/afk-companion-0.1.1.jar
```

Lizenz [MIT](../LICENSE).
