package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class ProtectModule extends Module {
    public ProtectModule() {
        super("Protect", "Bot defends you from hostile mobs.", Category.COMBAT);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.protect(""));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.stop());
    }
}
