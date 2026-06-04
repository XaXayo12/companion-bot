package com.afkcompanion.module;

import java.util.ArrayList;
import java.util.List;

import com.afkcompanion.module.setting.Setting;

import org.lwjgl.glfw.GLFW;

public abstract class Module {
    public final String name;
    public final String description;
    public final Category category;
    public final List<Setting> settings = new ArrayList<>();

    public boolean enabled;
    public int key = GLFW.GLFW_KEY_UNKNOWN;
    public boolean expanded;
    public boolean locked;

    protected Module(String name, String description, Category category) {
        this.name = name;
        this.description = description;
        this.category = category;
    }

    protected Setting add(Setting setting) {
        settings.add(setting);
        return setting;
    }

    public void toggle() {
        setEnabled(!enabled);
    }

    public void setEnabled(boolean value) {
        if (locked) {
            enabled = true;
            return;
        }
        if (enabled == value) {
            return;
        }
        enabled = value;
        if (enabled) {
            onEnable();
        } else {
            onDisable();
        }
    }

    protected void onEnable() {
    }

    protected void onDisable() {
    }

    public void onSettingChanged() {
    }
}
