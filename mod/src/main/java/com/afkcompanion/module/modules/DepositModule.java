package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class DepositModule extends Module {
    public DepositModule() {
        super("Deposit", "Bot empties its inventory into a nearby chest, then turns off.", Category.PLAYER);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.deposit());
        enabled = false;
    }
}
