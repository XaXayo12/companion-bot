package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class DropTrashModule extends Module {
    public DropTrashModule() {
        super("Drop Trash", "Bot drops junk items from its inventory, then turns off.", Category.MISC);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.dropTrash());
        enabled = false;
    }
}
