package com.afkcompanion.module.modules;

import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;

public class DeathAlertModule extends Module {
    public final Setting.Bool totemWarning;
    public final Setting.Slider lowHealth;

    public DeathAlertModule() {
        super("Death Alert", "On-screen warning when you are in danger or have no totem.", Category.PLAYER);
        totemWarning = (Setting.Bool) add(new Setting.Bool("Totem warning", true));
        lowHealth = (Setting.Slider) add(new Setting.Slider("Low-health warning", 6, 1, 20, 1));
        this.enabled = true;
    }
}
