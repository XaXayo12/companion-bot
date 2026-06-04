package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;
import com.afkcompanion.net.Protocol;

public class AutoEatModule extends Module {
    public final Setting.Slider eatBelow;

    public AutoEatModule() {
        super("Auto-Eat", "Always on. The bot eats by itself when its food drops to the level below.", Category.PLAYER);
        this.locked = true;
        this.enabled = true;
        eatBelow = (Setting.Slider) add(new Setting.Slider("Eat below (food)", 17, 1, 19, 1));
    }

    private void push() {
        AfkCompanionClient.bot.send(Protocol.eatThreshold((int) eatBelow.value));
    }

    @Override
    protected void onEnable() {
        push();
    }

    @Override
    public void onSettingChanged() {
        push();
    }
}
