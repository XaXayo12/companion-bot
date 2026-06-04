<div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=0:000000,100:dc2626&height=280&section=header&text=AFK%20Companion&fontSize=82&fontColor=ffffff&animation=fadeIn&fontAlignY=36&desc=Tu%20segunda%20cuenta%20de%20Minecraft,%20jugando%20mientras%20no%20estas&descAlignY=56&descSize=18&descColor=fca5a5" width="100%" alt="AFK Companion"/>

[![License: MIT](https://img.shields.io/badge/License-MIT-dc2626?style=for-the-badge&labelColor=000000)](../LICENSE)
[![Minecraft 1.21.11](https://img.shields.io/badge/Minecraft-1.21.11-dc2626?style=for-the-badge&logo=minecraft&logoColor=white&labelColor=000000)](https://www.minecraft.net/)
[![Fabric](https://img.shields.io/badge/Loader-Fabric-dc2626?style=for-the-badge&labelColor=000000)](https://fabricmc.net/)
[![Azalea](https://img.shields.io/badge/Bot-Azalea%200.15.1-dc2626?style=for-the-badge&labelColor=000000)](https://github.com/azalea-rs/azalea)

[English](../README.md) &nbsp; [Français](README.fr.md) &nbsp; **Español** &nbsp; [Deutsch](README.de.md) &nbsp; [Русский](README.ru.md) &nbsp; [Português](README.pt.md) &nbsp; [Italiano](README.it.md)

</div>

## Qué es

Un bot de segunda cuenta para **Minecraft 1.21.11** que se queda en el mundo por
ti. Te sigue, aleja a los monstruos, come cuando tiene hambre, mantiene un tótem
en la mano secundaria, mina una zona y puede traerte de vuelta con una cámara de
estasis. Lo controlas desde tu propio Minecraft con un pequeño menú (**F7**) o con
mensajes privados.

Tiene dos partes:

| Parte | Qué es | Dónde se ejecuta |
|-------|--------|------------------|
| **El bot** (`bot/`) | Un cliente real de Minecraft sin ventana, en Rust con [Azalea](https://github.com/azalea-rs/azalea). Conecta tu segunda cuenta y hace el trabajo. | Una consola en tu PC (Windows, macOS, Linux). |
| **El mod** (`mod/`) | Un pequeño mod [Fabric](https://fabricmc.net) para tu Minecraft. Dibuja el menú y las alertas y envía órdenes al bot. | Dentro de tu Minecraft normal. |

Solo se comunican dentro de tu ordenador (`ws://127.0.0.1:8765`).

> **Léelo.** Usar una segunda cuenta AFK infringe las reglas de muchos servidores.
> Usa AFK Companion solo en servidores que permitan cuentas alternativas. Eres
> responsable de tus cuentas.

## 1. Conseguir los archivos (sin compilar)

En la página de [**Releases**](../../releases), descarga:

1. El bot para tu sistema:
   * Windows: `afk-companion-bot-x86_64-pc-windows-msvc.exe`
   * Linux: `afk-companion-bot-x86_64-unknown-linux-gnu`
   * macOS (Apple Silicon): `afk-companion-bot-aarch64-apple-darwin`
2. El mod: `afk-companion-0.1.1.jar`

## 2. Instalar el mod

1. Instala **Fabric Loader** para Minecraft **1.21.11** (<https://fabricmc.net/use>).
2. Instala **Fabric API** para 1.21.11 (<https://modrinth.com/mod/fabric-api>).
3. Pon `afk-companion-0.1.1.jar` y el jar de Fabric API en `.minecraft/mods`.
4. Inicia Minecraft con el perfil de Fabric.

## 3. Iniciar el bot

Ejecuta el archivo del bot. En macOS y Linux: `chmod +x afk-companion-bot-*` y
luego `./afk-companion-bot-*`.

La primera vez hace unas preguntas sencillas (pulsa **Enter** para el valor entre
`[corchetes]`):

1. **Idioma:** `EN`, `FR`, `ES`, `DE`, `RU`, `PT` o `IT`.
2. **Tipo de cuenta:** `1` para **Microsoft** (por defecto) o `2` para
   cracked/sin conexión.
3. La **IP** y el **puerto** del servidor.

Nunca escribes tu correo ni un nombre de usuario. Para una cuenta Microsoft, el
bot muestra al arrancar un enlace y un código: ábrelo, escribe el código y
confirma. Solo una vez. (Solo una cuenta sin conexión pide un nombre, porque ese
nombre es su identidad.)

Guarda `config.json` a su lado y entra al servidor.

> **Consejo.** El bot y tú debéis estar en el mismo servidor. El bot solo puede
> seguirte, protegerte o tirarte si te ve en el mundo.

## 4. Jugar

* **F7**: abre el **menú de AFK Companion**. Haz clic para activar o desactivar.
  Con varios bots, pulsa una ficha de cuenta arriba para elegir cuál controlar, o
  `ALL`.
* **V**: pide al bot que te tire con una cámara de estasis.

Los comandos se dan solo por mensaje privado, así que los demás no los ven, y solo
**tú** (quien lanzó el juego con el mod) puedes comandar el bot:

```
/msg <bot> !follow    el bot te sigue
/msg <bot> !protect   el bot te protege y combate monstruos
/msg <bot> !come      el bot viene una vez
/msg <bot> !stop      detener todo
/msg <bot> !sel 1     marcar la esquina 1 (ponte donde quieras)
/msg <bot> !sel 2     marcar la esquina 2
/msg <bot> !mine      minar la caja entre las dos esquinas
/msg <bot> !pull      listar o activar cámaras de estasis cercanas
/msg <bot> !help      mostrar los comandos
```

El bot te responde en privado lo que hace, o el motivo exacto si no puede.

## Qué sabe hacer (el menú)

* **Combat:** KillAura (golpea monstruos a alcance legítimo, *Stay still* para
  granjas de spawner), Protect (te sigue y defiende).
* **Movement:** Follow (te sigue, distancia ajustable, evita la lava y los
  precipicios, sabe nadar), Hold Position, Goto Me, Sprint, Sneak.
* **Render:** HUD Info, Bot Tracer, Background.
* **Player:** Auto-Eat (siempre activo, come cuando la comida baja al nivel
  configurado, por defecto 17), Auto-Armor, Auto-Totem, Auto-Disconnect,
  Auto-Pull, Death Alert.
* **Misc:** Notifications (activo por defecto, muestra los cambios de estado y
  avisos como "I can't see you" en tu chat), Deposit, Drop Trash, Mine.

## El archivo `config.json`

```json
{
  "server": "play.example.com",
  "data_dir": ".afk",
  "language": "es",
  "owner": "",
  "bridge_port": 8765,
  "reconnect_seconds": 8,
  "accounts": [
    { "username": "account-1", "auth": "microsoft" }
  ]
}
```

* `server`: el servidor. `language`: `en`, `fr`, `es`, `de`, `ru`, `pt`, `it`.
* `owner`: déjalo vacío, el mod envía tu nombre automáticamente. Rellénalo solo si
  usas el bot sin el mod.
* `accounts`: para Microsoft, `username` es solo una etiqueta local del login
  guardado; para `offline` es el nombre en el juego. Añade varias entradas para
  varios bots a la vez.

## Solución de problemas

* **"Waiting for the bot"**: el bot no está en marcha o no es accesible. Comprueba
  que su consola está abierta y que `bridge_port` coincide.
* **Follow no hace nada, o "I can't see you"**: el bot solo actúa en el mismo
  servidor y dentro de su distancia de renderizado. Únete al mismo servidor y
  acércate. Activa el módulo **Notifications** si no ves el mensaje.
* **Mi comando susurrado no hizo nada**: solo quien lanzó el juego con el mod
  puede comandar el bot, y solo por `/msg`. Asegúrate de susurrar desde tu cuenta,
  no en el chat público.
* **Falló el inicio de sesión de Microsoft**: abre el enlace exacto, escribe el
  código y confirma con la cuenta correcta. Borra el archivo en `.afk`, o *log
  out* en el menú, para empezar de nuevo.

## Compilar tú mismo

```bash
cd bot && cargo run            # bot (toolchain Rust nightly)
cd mod && ./gradlew build      # mod (JDK 21), salida build/libs/afk-companion-0.1.1.jar
```

Licencia [MIT](../LICENSE).
