package com.afkcompanion.gui;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.font.TextRenderer;
import net.minecraft.client.gui.DrawContext;

public class RenderUtil {
    public static void rect(DrawContext context, int x, int y, int width, int height, int color) {
        context.fill(x, y, x + width, y + height, color);
    }

    public static void roundedRect(DrawContext context, int x, int y, int width, int height, int color) {
        int r = 2;
        context.fill(x + r, y, x + width - r, y + height, color);
        context.fill(x, y + r, x + r, y + height - r, color);
        context.fill(x + width - r, y + r, x + width, y + height - r, color);
    }

    public static void text(DrawContext context, String value, int x, int y, int color) {
        TextRenderer renderer = MinecraftClient.getInstance().textRenderer;
        context.drawText(renderer, value, x, y, color, true);
    }

    public static int width(String value) {
        return MinecraftClient.getInstance().textRenderer.getWidth(value);
    }

    public static int fontHeight() {
        return MinecraftClient.getInstance().textRenderer.fontHeight;
    }
}
