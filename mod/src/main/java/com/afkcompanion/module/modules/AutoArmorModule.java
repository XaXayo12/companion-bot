package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class AutoArmorModule extends Module {
    public AutoArmorModule() {
        super("Auto-Armor", "Bot equips the best armor it has and replaces broken pieces.", Category.PLAYER);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.autoArmor(true));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.autoArmor(false));
    }
}
