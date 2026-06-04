<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Tvoy%20vtoroy%20akkaunt%20Minecraft,%20igrayushchiy%20poka%20tebya%20net&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; [Français](README.fr.md) &nbsp; [Español](README.es.md) &nbsp; [Deutsch](README.de.md) &nbsp; **Русский** &nbsp; [Português](README.pt.md) &nbsp; [Italiano](README.it.md)

</div>

## Что это

Бот второго аккаунта для **Minecraft 1.21.11**, который остаётся в мире вместо
тебя. Следует за тобой, отбивает мобов, ест когда голоден, держит тотем во второй
руке, копает зону и может вытащить тебя домой через стазис-камеру. Управляешь им
из своего Minecraft маленьким меню (**F7**) или личными сообщениями.

Состоит из двух частей:

| Часть | Что это | Где работает |
|-------|---------|--------------|
| **Бот** (`bot/`) | Настоящий клиент Minecraft без окна, на Rust поверх [Azalea](https://github.com/azalea-rs/azalea). Заходит твоим вторым аккаунтом и делает работу. | Консоль на твоём ПК (Windows, macOS, Linux). |
| **Мод** (`mod/`) | Небольшой мод [Fabric](https://fabricmc.net) для твоего Minecraft. Рисует меню и оповещения и шлёт боту команды. | Внутри обычного Minecraft. |

Они общаются только внутри твоего компьютера (`ws://127.0.0.1:8765`).

> **Прочти.** Второй AFK-аккаунт запрещён правилами многих серверов. Используй AFK
> Companion только там, где разрешены дополнительные аккаунты. Ты отвечаешь за свои
> аккаунты.

## 1. Скачать файлы (без компиляции)

На странице [**Releases**](../../releases) скачай:

1. Бота под свою систему:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. Мод: `afk-companion-0.1.1.jar`

## 2. Установить мод

1. Установи **Fabric Loader** для Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Установи **Fabric API** для 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Положи `afk-companion-0.1.1.jar` и jar Fabric API в `.minecraft/mods`.
4. Запусти Minecraft с профилем Fabric.

## 3. Запустить бота

Запусти файл бота. На macOS и Linux: `chmod +x afk-companion-bot-*`, затем
`./afk-companion-bot-*`.

При первом запуске он задаёт несколько простых вопросов (нажми **Enter** для
значения в `[скобках]`):

1. **Язык:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` или `IT`.
2. **Тип аккаунта:** `1` для **Microsoft** (по умолчанию) или `2` для
   cracked/офлайн.
3. **IP** и **порт** сервера.

Ты никогда не вводишь свою почту или имя. Для аккаунта Microsoft бот при запуске
показывает ссылку и код: открой ссылку, введи код, подтверди. Только один раз.
(Только офлайн-аккаунт спрашивает имя, потому что это имя и есть его личность.)

Он пишет рядом `config.json` и заходит на сервер.

> **Совет.** Бот и ты должны быть на одном сервере. Бот может следовать, защищать
> или тащить тебя только если видит тебя в мире.

## 4. Играть

* **F7**: открывает **меню AFK Companion**. Клик включает или выключает. С
  несколькими ботами кликни наверху фишку аккаунта, чтобы выбрать кем управлять,
  или `ALL`.
* **V**: просит бота вытащить тебя через стазис-камеру.

Команды даются только личным сообщением, поэтому другие их не видят, и только **ты**
(кто запустил игру с модом) можешь командовать ботом:

```
/msg <bot> !follow    бот следует за тобой
/msg <bot> !protect   бот защищает тебя и бьёт мобов
/msg <bot> !come      бот приходит один раз
/msg <bot> !stop      остановить всё
/msg <bot> !sel 1     отметить угол 1 (встань где нужно)
/msg <bot> !sel 2     отметить угол 2
/msg <bot> !mine      копать область между углами
/msg <bot> !pull      список или активация ближних стазис-камер
/msg <bot> !help      показать команды
```

Бот отвечает в личку, что делает, или точную причину, если не может.

## Что умеет (меню)

* **Combat:** KillAura (бьёт мобов в честном радиусе, *Stay still* для ферм
  спавнеров), Protect (следует и защищает).
* **Movement:** Follow (следует, регулируемая дистанция, избегает лавы и обрывов,
  умеет плавать), Hold Position, Goto Me, Sprint, Sneak.
* **Render:** HUD Info, Bot Tracer, Background.
* **Player:** Auto-Eat (всегда вкл, ест когда сытость падает до заданного уровня,
  по умолчанию 17), Auto-Armor, Auto-Totem, Auto-Disconnect, Auto-Pull, Death
  Alert.
* **Misc:** Notifications (вкл по умолчанию, показывает смену статуса и
  предупреждения вроде "I can't see you" в чате), Deposit, Drop Trash, Mine.

## Файл `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "ru",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server`: сервер. `language`: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`.
* `owner`: оставь пустым, мод шлёт твоё имя автоматически. Заполняй только если
  используешь бота без мода.
* `accounts`: для Microsoft `username` это просто локальная метка для сохранённого
  входа; для `offline` это имя в игре. Добавь несколько записей для нескольких
  ботов сразу.

## Если что-то не так

* **"Waiting for the bot"**: бот не запущен или недоступен. Проверь, что консоль
  бота открыта и `bridge_port` совпадает.
* **Follow ничего не делает, или "I can't see you"**: бот действует только на том
  же сервере и в пределах дальности прорисовки. Зайди на тот же сервер и подойди
  ближе. Включи модуль **Notifications**, если не видишь сообщение.
* **Моя команда шёпотом ничего не сделала**: только тот, кто запустил игру с
  модом, может командовать ботом, и только через `/msg`. Убедись, что шепчешь со
  своего аккаунта, а не в общий чат.
* **Не вышел вход Microsoft**: открой точную ссылку, введи код, подтверди нужным
  аккаунтом. Удали файл в `.afk`, или *log out* в меню, чтобы начать заново.

## Собрать самому

```bash
cd bot && cargo run            # бот (Rust nightly)
cd mod && ./gradlew build      # мод (JDK 21), вывод build/libs/afk-companion-0.1.1.jar
```

Лицензия [MIT](../LICENSE).
