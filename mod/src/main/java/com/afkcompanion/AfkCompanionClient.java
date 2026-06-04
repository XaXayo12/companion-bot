package com.afkcompanion;

import com.afkcompanion.config.ModConfig;
import com.afkcompanion.gui.ClickGuiScreen;
import com.afkcompanion.gui.Hud;
import com.afkcompanion.input.KeybindHandler;
import com.afkcompanion.input.Keybinds;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.ModuleManager;
import com.afkcompanion.net.BotLink;
import com.afkcompanion.net.Protocol;
import com.afkcompanion.sensor.DangerSensor;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.render.RenderTickCounter;
import net.minecraft.text.Text;
import net.minecraft.util.Formatting;

public class AfkCompanionClient implements ClientModInitializer {

    public static ModConfig config;
    public static ModuleManager modules;
    public static BotLink bot;
    public static DangerSensor danger;

    // Tracks the owner name we last told the bot, so we only send it when it
    // changes or after a reconnect. null means "not sent on this link yet".
    private String lastOwnerSent;
    private boolean wasLinked;

    @Override
    public void onInitializeClient() {
        config = ModConfig.load();
        modules = new ModuleManager();
        bot = new BotLink(config.botHost, config.botPort);
        danger = new DangerSensor();

        Keybinds.register();
        ClientTickEvents.END_CLIENT_TICK.register(this::onClientTick);
        HudRenderCallback.EVENT.register(this::onHudRender);

        bot.connect();
        AfkCompanion.LOG.info("{} ready. Press F7 in-game to open the menu.", AfkCompanion.MOD_NAME);
    }

    private void onClientTick(MinecraftClient client) {
        try {
            while (Keybinds.openGui.wasPressed()) {
                client.setScreen(new ClickGuiScreen());
            }
            while (Keybinds.pullMe.wasPressed()) {
                bot.send(Protocol.pull("manual"));
            }
            KeybindHandler.tick(client);
            bot.tick();
            sendOwnerIfNeeded(client);
            drainNotifications(client);
            danger.tick(client);
        } catch (Throwable t) {
            AfkCompanion.LOG.error("Client tick error (ignored, game keeps running)", t);
        }
    }

    // Automatically tell the bot your Minecraft name so it only obeys whispers
    // from you. No need to type an "owner" anywhere: the mod knows who you are.
    // Re-sent after each (re)connect in case the bot restarted.
    private void sendOwnerIfNeeded(MinecraftClient client) {
        boolean linked = bot.linked();
        if (!linked) {
            wasLinked = false;
            lastOwnerSent = null;
            return;
        }
        if (!wasLinked) {
            wasLinked = true;
            lastOwnerSent = null;
        }
        if (client.getSession() == null) {
            return;
        }
        String name = client.getSession().getUsername();
        if (name != null && !name.isEmpty() && !name.equals(lastOwnerSent)) {
            bot.send(Protocol.setOwner(name));
            lastOwnerSent = name;
        }
    }

    // Show the bot's queued notices (status changes and errors, e.g. "I can't
    // see you" when the bot is on another server) in your chat. Always drains
    // the queue so it can't grow without bound; only prints when the
    // Notifications module is on (it is by default).
    private void drainNotifications(MinecraftClient client) {
        Module notifications = modules.byName("Notifications");
        boolean show = notifications != null && notifications.enabled;
        BotLink.Notice notice;
        while ((notice = bot.pollNotice()) != null) {
            if (show && client.player != null && client.inGameHud != null) {
                Text text = Text.literal(notice.text())
                        .formatted(notice.error() ? Formatting.RED : Formatting.GRAY);
                client.inGameHud.getChatHud().addMessage(text);
            }
        }
    }

    private void onHudRender(DrawContext context, RenderTickCounter counter) {
        try {
            Hud.render(context);
        } catch (Throwable ignored) {
        }
    }
}
