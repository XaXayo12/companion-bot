package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class AutoTotemModule extends Module {
    public AutoTotemModule() {
        super("Auto-Totem", "Bot keeps a totem of undying in its off-hand.", Category.COMBAT);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.autoTotem(true));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.autoTotem(false));
    }
}
