package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;
import com.afkcompanion.net.Protocol;

public class KillAuraModule extends Module {
    public final Setting.Slider range;
    public final Setting.Bool stayStill;

    public KillAuraModule() {
        super("KillAura", "Bot attacks hostile mobs that are in legit reach.", Category.COMBAT);
        range = (Setting.Slider) add(new Setting.Slider("Range", 3.0, 2.0, 4.0, 0.1));
        stayStill = (Setting.Bool) add(new Setting.Bool("Stay still (spawner farm)", true));
    }

    private void push() {
        AfkCompanionClient.bot.send(Protocol.killAura(enabled, range.value, stayStill.value));
    }

    @Override
    protected void onEnable() {
        push();
    }

    @Override
    protected void onDisable() {
        push();
    }

    @Override
    public void onSettingChanged() {
        if (enabled) {
            push();
        }
    }
}
