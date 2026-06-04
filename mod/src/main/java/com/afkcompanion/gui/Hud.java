package com.afkcompanion.gui;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.modules.DeathAlertModule;
import com.afkcompanion.net.BotState;
import com.afkcompanion.sensor.DangerSensor;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.network.ClientPlayerEntity;

public class Hud {
    public static void render(DrawContext context) {
        MinecraftClient client = MinecraftClient.getInstance();
        if (client.player == null || client.currentScreen != null) {
            return;
        }
        if (client.options.hudHidden) {
            return;
        }

        var link = AfkCompanionClient.bot;
        boolean linked = link.linked();
        BotState state = link.selectedState();
        int accountCount = link.botIds().size();

        Module hudInfo = AfkCompanionClient.modules.byName("HUD Info");
        if (hudInfo != null && hudInfo.enabled) {
            String who = state.id.isEmpty() ? "Bot" : "Bot " + state.id;
            String line1 = linked ? who + ": " + state.status : "Bot: offline";
            if (linked && accountCount > 1) {
                line1 += "  (" + accountCount + " accounts, " + link.selected() + " selected)";
            }

            // Collect the lines first so we can size a readable background panel.
            java.util.List<String> lines = new java.util.ArrayList<>();
            java.util.List<Integer> colors = new java.util.ArrayList<>();
            lines.add(line1);
            colors.add(linked ? Theme.accent : Theme.TEXT_DIM);
            if (linked) {
                lines.add(String.format("HP %.0f   Food %d", state.health, state.food));
                colors.add(Theme.TEXT);
                lines.add(String.format("Mode %s   Dist %.1fm", state.mode, distanceTo(client.player, state)));
                colors.add(Theme.TEXT_DIM);
                String log = state.lastLog == null ? "" : state.lastLog.trim();
                if (log.length() > 38) {
                    log = log.substring(0, 37) + "…";
                }
                lines.add(log.isEmpty() ? "Idle." : log);
                colors.add(Theme.TEXT_DIM);
            }

            int width = 0;
            for (String s : lines) {
                width = Math.max(width, RenderUtil.width(s));
            }
            RenderUtil.rect(context, 2, 2, width + 8, lines.size() * 10 + 4, Theme.PANEL);
            // A small accent stripe marks online (green-ish) vs offline (dim).
            RenderUtil.rect(context, 2, 2, 2, lines.size() * 10 + 4, linked ? Theme.ONLINE : Theme.TEXT_DIM);
            for (int i = 0; i < lines.size(); i++) {
                RenderUtil.text(context, lines.get(i), 8, 6 + i * 10, colors.get(i));
            }
        }

        Module tracer = AfkCompanionClient.modules.byName("Bot Tracer");
        if (tracer != null && tracer.enabled && linked) {
            renderTracer(context, client, state);
        }

        DangerSensor sensor = AfkCompanionClient.danger;
        Module alert = AfkCompanionClient.modules.byName("Death Alert");
        if (alert instanceof DeathAlertModule deathAlert && deathAlert.enabled) {
            int centerX = client.getWindow().getScaledWidth() / 2;
            int y = client.getWindow().getScaledHeight() / 2 - 40;
            if (deathAlert.totemWarning.value && sensor.noTotem) {
                String warn = "NO TOTEM";
                RenderUtil.text(context, warn, centerX - RenderUtil.width(warn) / 2, y, Theme.WARN);
            }
            if (sensor.inDanger) {
                String warn = "DANGER: " + sensor.reason;
                RenderUtil.text(context, warn, centerX - RenderUtil.width(warn) / 2, y + 11, Theme.WARN);
            }
        }
    }

    private static double distanceTo(ClientPlayerEntity player, BotState state) {
        double dx = state.x - player.getX();
        double dy = state.y - player.getY();
        double dz = state.z - player.getZ();
        return Math.sqrt(dx * dx + dy * dy + dz * dz);
    }

    private static void renderTracer(DrawContext context, MinecraftClient client, BotState state) {
        ClientPlayerEntity player = client.player;
        double dx = state.x - player.getX();
        double dz = state.z - player.getZ();
        double bearing = Math.toDegrees(Math.atan2(-dx, dz));
        double relative = Math.toRadians(bearing - player.getYaw());

        int centerX = client.getWindow().getScaledWidth() / 2;
        int centerY = client.getWindow().getScaledHeight() / 2;
        int radius = 40;
        int x = centerX + (int) Math.round(Math.sin(relative) * radius);
        int y = centerY - (int) Math.round(Math.cos(relative) * radius);

        RenderUtil.rect(context, x - 1, y - 1, 3, 3, Theme.accent);
        String label = String.format("%.0fm", distanceTo(player, state));
        RenderUtil.text(context, label, x - RenderUtil.width(label) / 2, y + 4, Theme.accent);
    }
}
