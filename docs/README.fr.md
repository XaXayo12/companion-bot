<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Ton%20second%20compte%20Minecraft,%20qui%20joue%20quand%20tu%20es%20absent&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; **Français** &nbsp; [Español](README.es.md) &nbsp; [Deutsch](README.de.md) &nbsp; [Русский](README.ru.md) &nbsp; [Português](README.pt.md) &nbsp; [Italiano](README.it.md)

</div>

## Ce que c'est

Un bot de second compte pour **Minecraft 1.21.11** qui reste dans le monde à ta
place. Il te suit, repousse les monstres, mange quand il a faim, garde une totem
en main secondaire, mine une zone et peut te ramener avec une chambre de stase.
Tu le pilotes depuis ton propre Minecraft avec un petit menu (**F7**) ou par
messages privés.

Il est en deux parties :

| Partie | Ce que c'est | Où ça tourne |
|--------|--------------|--------------|
| **Le bot** (`bot/`) | Un vrai client Minecraft sans fenêtre, en Rust avec [Azalea](https://github.com/azalea-rs/azalea). Il connecte ton second compte et fait le travail. | Une console sur ton PC (Windows, macOS, Linux). |
| **Le mod** (`mod/`) | Un petit mod [Fabric](https://fabricmc.net) pour ton Minecraft. Il dessine le menu et les alertes et envoie les ordres au bot. | Dans ton Minecraft normal. |

Ils se parlent uniquement sur ton ordinateur (`ws://127.0.0.1:8765`).

> **À lire.** Utiliser un second compte AFK est interdit sur beaucoup de serveurs.
> N'utilise AFK Companion que sur des serveurs qui autorisent les comptes
> secondaires. Tu es responsable de tes comptes.

## 1. Récupérer les fichiers (rien à compiler)

Sur la page [**Releases**](../../releases), télécharge :

1. Le bot pour ton système :
   * Windows : `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux : `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon) : `afk-companion-bot-aarch64-apple-darwin`
2. Le mod : `afk-companion-0.1.1.jar`

## 2. Installer le mod

1. Installe **Fabric Loader** pour Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Installe **Fabric API** pour 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Mets `afk-companion-0.1.1.jar` et le jar de Fabric API dans `.minecraft/mods`.
4. Lance Minecraft avec le profil Fabric.

## 3. Lancer le bot

Lance le fichier du bot. Sur macOS et Linux : `chmod +x afk-companion-bot-*` puis
`./afk-companion-bot-*`.

Au premier lancement, il pose quelques questions simples (appuie sur **Entrée**
pour la valeur entre `[crochets]`) :

1. **Langue :** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` ou `IT`.
2. **Type de compte :** `1` pour **Microsoft** (par défaut) ou `2` pour
   cracké/hors-ligne.
3. L'**IP** et le **port** du serveur.

Tu ne tapes jamais ton e-mail ni un pseudo. Pour un compte Microsoft, le bot
affiche au démarrage un lien et un code : ouvre le lien, tape le code, valide.
C'est une seule fois. (Seul un compte hors-ligne demande un nom, car ce nom est
son identité.)

Il écrit `config.json` à côté de lui, puis rejoint le serveur.

> **Astuce.** Le bot et toi devez être sur le même serveur. Le bot ne peut te
> suivre, te protéger ou te tirer que s'il te voit dans le monde.

## 4. Jouer

* **F7** : ouvre le **menu AFK Companion**. Clique pour activer ou désactiver.
  Avec plusieurs bots, clique une pastille de compte en haut pour choisir lequel
  contrôler, ou `ALL`.
* **V** : demande au bot de te tirer avec une chambre de stase.

Les commandes se donnent uniquement en message privé, donc les autres joueurs ne
les voient pas, et seul **toi** (le joueur qui a lancé le jeu avec le mod) peux
commander le bot :

```
/msg <bot> !follow    le bot te suit
/msg <bot> !protect   le bot te garde et combat les monstres
/msg <bot> !come      le bot vient une fois
/msg <bot> !stop      tout arrêter
/msg <bot> !sel 1     marquer le coin 1 (mets-toi où tu veux)
/msg <bot> !sel 2     marquer le coin 2
/msg <bot> !mine      miner la zone entre les deux coins
/msg <bot> !pull      lister ou activer les chambres de stase proches
/msg <bot> !help      afficher les commandes
```

Le bot te répond en privé ce qu'il fait, ou la raison exacte s'il ne peut pas.

## Ce qu'il sait faire (le menu)

* **Combat :** KillAura (frappe les monstres à portée légitime, *Rester immobile*
  pour les farms de spawner), Protect (te suit et défend).
* **Mouvement :** Follow (te suit, distance réglable, évite la lave et les
  falaises, sait nager), Hold Position, Goto Me, Sprint, Sneak.
* **Render :** HUD Info, Bot Tracer, Background.
* **Player :** Auto-Eat (toujours actif, mange quand la faim tombe au niveau
  réglé, défaut 17), Auto-Armor, Auto-Totem, Auto-Disconnect, Auto-Pull, Death
  Alert.
* **Misc :** Notifications (actif par défaut, affiche les changements de statut et
  les avertissements comme "I can't see you" dans ton chat), Deposit, Drop Trash,
  Mine.

## Le fichier `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "fr",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server` : le serveur à rejoindre. `language` : `en`, `fr`, `es`, `de`, `ru`,
  `pt`, `it`.
* `owner` : à laisser vide, le mod envoie ton nom automatiquement. À remplir
  seulement si tu utilises le bot sans le mod.
* `accounts` : pour Microsoft, `username` n'est qu'un libellé local pour le login
  enregistré ; pour `offline` c'est le nom en jeu. Ajoute plusieurs entrées pour
  plusieurs bots à la fois.

## Dépannage

* **"Waiting for the bot"** : le bot n'est pas lancé ou pas joignable. Vérifie que
  sa console est ouverte et que `bridge_port` correspond.
* **Follow ne fait rien, ou "I can't see you"** : le bot n'agit que sur le même
  serveur et dans sa render distance. Rejoins le même serveur et rapproche-toi.
  Active le module **Notifications** si tu ne vois pas le message.
* **Ma commande chuchotée n'a rien fait** : seul le joueur qui a lancé le jeu avec
  le mod peut commander le bot, et seulement par `/msg`. Vérifie que tu chuchotes
  depuis ton compte, pas en chat public.
* **Connexion Microsoft échouée** : ouvre le lien exact, tape le code, valide avec
  le bon compte. Supprime le fichier dans `.afk`, ou *log out* dans le menu, pour
  recommencer.

## Compiler soi-même

```bash
cd bot && cargo run            # bot (toolchain Rust nightly)
cd mod && ./gradlew build      # mod (JDK 21), sortie build/libs/afk-companion-0.1.1.jar
```

Licence [MIT](../LICENSE).
