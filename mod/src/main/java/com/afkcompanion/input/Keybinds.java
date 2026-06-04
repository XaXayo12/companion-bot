package com.afkcompanion.input;

import com.afkcompanion.AfkCompanion;

import net.fabricmc.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.minecraft.client.option.KeyBinding;
import net.minecraft.client.util.InputUtil;
import net.minecraft.util.Identifier;

import org.lwjgl.glfw.GLFW;

public class Keybinds {
    private static final KeyBinding.Category CATEGORY =
            KeyBinding.Category.create(Identifier.of(AfkCompanion.MOD_ID, "main"));

    public static KeyBinding openGui;
    public static KeyBinding pullMe;

    public static void register() {
        // F7 is the default GUI key. Vanilla Minecraft 1.21.11 leaves F7
        // unbound (its only default F-key binds are F1 hide-HUD, F2 screenshot,
        // F3 debug, F5 perspective, F11 fullscreen; the F3 chords use F3+key).
        // Because this is registered as a real Fabric KeyBinding it also appears
        // in Options > Controls, where Minecraft flags any conflict with another
        // mod and lets the player rebind it.
        openGui = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key." + AfkCompanion.MOD_ID + ".open_gui",
                InputUtil.Type.KEYSYM,
                GLFW.GLFW_KEY_F7,
                CATEGORY));

        pullMe = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key." + AfkCompanion.MOD_ID + ".pull_me",
                InputUtil.Type.KEYSYM,
                GLFW.GLFW_KEY_V,
                CATEGORY));
    }
}
