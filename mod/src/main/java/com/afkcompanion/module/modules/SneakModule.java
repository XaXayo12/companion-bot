package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class SneakModule extends Module {
    public SneakModule() {
        super("Sneak", "Bot stays sneaking so it will not walk off edges.", Category.MOVEMENT);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.sneak(true));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.sneak(false));
    }
}
