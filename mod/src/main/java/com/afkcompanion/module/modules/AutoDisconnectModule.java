package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;
import com.afkcompanion.net.Protocol;

public class AutoDisconnectModule extends Module {
    public final Setting.Slider health;

    public AutoDisconnectModule() {
        super("Auto-Disconnect", "Bot logs out when its health drops below the threshold.", Category.COMBAT);
        health = (Setting.Slider) add(new Setting.Slider("Health", 4, 1, 20, 1));
    }

    private void push() {
        AfkCompanionClient.bot.send(Protocol.autoDisconnect(enabled, health.value));
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
