package com.afkcompanion.input;

import java.util.HashSet;
import java.util.Set;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Module;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.util.InputUtil;

import org.lwjgl.glfw.GLFW;

public class KeybindHandler {
    private static final Set<Integer> held = new HashSet<>();

    public static void tick(MinecraftClient client) {
        if (client.currentScreen != null) {
            return;
        }
        var window = client.getWindow();
        for (Module module : AfkCompanionClient.modules.all()) {
            if (module.key == GLFW.GLFW_KEY_UNKNOWN) {
                continue;
            }
            boolean pressed = InputUtil.isKeyPressed(window, module.key);
            if (pressed && !held.contains(module.key)) {
                module.toggle();
            }
            if (pressed) {
                held.add(module.key);
            } else {
                held.remove(module.key);
            }
        }
    }
}
