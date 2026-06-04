package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class HoldPositionModule extends Module {
    public HoldPositionModule() {
        super("Hold Position", "Bot stops moving and stays where it is.", Category.MOVEMENT);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.holdPosition(true));
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.holdPosition(false));
    }
}
