package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;
import com.afkcompanion.net.Protocol;

public class FollowModule extends Module {
    public final Setting.Slider distance;

    public FollowModule() {
        super("Follow", "Bot follows you (it refuses to follow into lava or off cliffs).", Category.MOVEMENT);
        distance = (Setting.Slider) add(new Setting.Slider("Distance", 2, 1, 10, 1));
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.follow(""));
        AfkCompanionClient.bot.send(Protocol.followDistance(distance.value));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.stop());
    }

    @Override
    public void onSettingChanged() {
        if (enabled) {
            AfkCompanionClient.bot.send(Protocol.followDistance(distance.value));
        }
    }
}
