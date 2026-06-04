<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Sua%20segunda%20conta%20de%20Minecraft,%20jogando%20enquanto%20voce%20esta%20fora&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; [Français](README.fr.md) &nbsp; [Español](README.es.md) &nbsp; [Deutsch](README.de.md) &nbsp; [Русский](README.ru.md) &nbsp; **Português** &nbsp; [Italiano](README.it.md)

</div>

## O que é

Um bot de segunda conta para **Minecraft 1.21.11** que fica no mundo por você. Ele
segue você, afasta os monstros, come quando está com fome, mantém um totem na mão
secundária, minera uma área e pode te puxar de volta com uma câmara de estase.
Você o controla a partir do seu próprio Minecraft com um pequeno menu (**F7**) ou
com mensagens privadas.

Tem duas partes:

| Parte | O que é | Onde roda |
|-------|---------|-----------|
| **O bot** (`bot/`) | Um cliente real de Minecraft sem janela, em Rust com [Azalea](https://github.com/azalea-rs/azalea). Conecta sua segunda conta e faz o trabalho. | Um console no seu PC (Windows, macOS, Linux). |
| **O mod** (`mod/`) | Um pequeno mod [Fabric](https://fabricmc.net) para o seu Minecraft. Desenha o menu e os alertas e envia ordens ao bot. | Dentro do seu Minecraft normal. |

Eles só conversam dentro do seu computador (`ws://127.0.0.1:8765`).

> **Leia.** Usar uma segunda conta AFK viola as regras de muitos servidores. Use o
> AFK Companion apenas em servidores que permitem contas alternativas. Você é
> responsável pelas suas contas.

## 1. Pegar os arquivos (sem compilar)

Na página de [**Releases**](../../releases), baixe:

1. O bot para o seu sistema:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. O mod: `afk-companion-0.1.1.jar`

## 2. Instalar o mod

1. Instale o **Fabric Loader** para Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Instale a **Fabric API** para 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Coloque `afk-companion-0.1.1.jar` e o jar da Fabric API em `.minecraft/mods`.
4. Inicie o Minecraft com o perfil Fabric.

## 3. Iniciar o bot

Execute o arquivo do bot. No macOS e Linux: `chmod +x afk-companion-bot-*` e depois
`./afk-companion-bot-*`.

Na primeira vez, ele faz algumas perguntas simples (aperte **Enter** para o valor
entre `[colchetes]`):

1. **Idioma:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` ou `IT`.
2. **Tipo de conta:** `1` para **Microsoft** (padrão) ou `2` para cracked/offline.
3. O **IP** e a **porta** do servidor.

Você nunca digita seu e-mail nem um nome de usuário. Para uma conta Microsoft, o
bot mostra ao iniciar um link e um código: abra o link, digite o código, confirme.
Só uma vez. (Só uma conta offline pede um nome, porque esse nome é a identidade
dela.)

Ele grava `config.json` ao lado e entra no servidor.

> **Dica.** O bot e você devem estar no mesmo servidor. O bot só pode seguir,
> proteger ou puxar você se conseguir te ver no mundo.

## 4. Jogar

* **F7**: abre o **menu do AFK Companion**. Clique para ligar ou desligar. Com
  vários bots, clique numa ficha de conta no topo para escolher qual controlar, ou
  `ALL`.
* **V**: pede ao bot para te puxar com uma câmara de estase.

Os comandos são dados apenas por mensagem privada, então os outros não os veem, e
só **você** (quem iniciou o jogo com o mod) pode comandar o bot:

```
/msg <bot> !follow    o bot segue você
/msg <bot> !protect   o bot protege você e luta contra monstros
/msg <bot> !come      o bot vem uma vez
/msg <bot> !stop      parar tudo
/msg <bot> !sel 1     marcar o canto 1 (fique onde quiser)
/msg <bot> !sel 2     marcar o canto 2
/msg <bot> !mine      minerar a caixa entre os dois cantos
/msg <bot> !pull      listar ou ativar câmaras de estase próximas
/msg <bot> !help      mostrar os comandos
```

O bot responde em privado o que está fazendo, ou o motivo exato se não puder.

## O que ele faz (o menu)

* **Combat:** KillAura (ataca monstros ao alcance legítimo, *Stay still* para farms
  de spawner), Protect (segue e defende você).
* **Movement:** Follow (segue você, distância ajustável, evita lava e penhascos,
  sabe nadar), Hold Position, Goto Me, Sprint, Sneak.
* **Render:** HUD Info, Bot Tracer, Background.
* **Player:** Auto-Eat (sempre ligado, come quando a fome cai ao nível definido,
  padrão 17), Auto-Armor, Auto-Totem, Auto-Disconnect, Auto-Pull, Death Alert.
* **Misc:** Notifications (ligado por padrão, mostra mudanças de status e avisos
  como "I can't see you" no seu chat), Deposit, Drop Trash, Mine.

## O arquivo `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "pt",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server`: o servidor. `language`: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`.
* `owner`: deixe vazio, o mod envia seu nome automaticamente. Preencha só se usar o
  bot sem o mod.
* `accounts`: para Microsoft, `username` é apenas um rótulo local do login salvo;
  para `offline` é o nome no jogo. Adicione várias entradas para vários bots ao
  mesmo tempo.

## Solução de problemas

* **"Waiting for the bot"**: o bot não está rodando ou não está acessível. Verifique
  se o console do bot está aberto e se `bridge_port` corresponde.
* **Follow não faz nada, ou "I can't see you"**: o bot só age no mesmo servidor e
  dentro da sua distância de renderização. Entre no mesmo servidor e aproxime-se.
  Ative o módulo **Notifications** se não vir a mensagem.
* **Meu comando sussurrado não fez nada**: só quem iniciou o jogo com o mod pode
  comandar o bot, e só por `/msg`. Confirme que você sussurra da sua conta, não no
  chat público.
* **O login da Microsoft falhou**: abra o link exato, digite o código, confirme com
  a conta certa. Apague o arquivo em `.afk`, ou *log out* no menu, para recomeçar.

## Compilar você mesmo

```bash
cd bot && cargo run            # bot (toolchain Rust nightly)
cd mod && ./gradlew build      # mod (JDK 21), saída build/libs/afk-companion-0.1.1.jar
```

Licença [MIT](../LICENSE).
