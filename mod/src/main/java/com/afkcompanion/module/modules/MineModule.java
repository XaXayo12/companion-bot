package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

public class MineModule extends Module {
    public MineModule() {
        super("Mine Area", "Bot mines between two points. Whisper /msg <bot> !sel 1 and !sel 2 to set them.", Category.MISC);
    }

    @Override
    protected void onEnable() {
        AfkCompanionClient.bot.send(Protocol.mineStart());
    }

    @Override
    protected void onDisable() {
        AfkCompanionClient.bot.send(Protocol.stop());
    }
}
