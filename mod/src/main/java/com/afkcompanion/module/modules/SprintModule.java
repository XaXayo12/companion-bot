package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class SprintModule extends Module {
    public SprintModule() {
        super("Sprint", "Bot sprints while moving.", Category.MOVEMENT);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.sprint(true));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.sprint(false));
    }
}
