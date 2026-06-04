package com.afkcompanion.module.modules;

import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;

public class AutoPullModule extends Module {
    public final Setting.Bool onLowHealth;
    public final Setting.Slider healthThreshold;

    public AutoPullModule() {
        super("Auto-Pull", "Asks the bot to pull you (stasis) when your health drops.", Category.PLAYER);
        onLowHealth = (Setting.Bool) add(new Setting.Bool("On low health", true));
        healthThreshold = (Setting.Slider) add(new Setting.Slider("Health threshold", 6, 1, 20, 1));
    }
}
