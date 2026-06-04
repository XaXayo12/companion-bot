package com.afkcompanion.module.modules;

import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;

public class HudInfoModule extends Module {
    public HudInfoModule() {
        super("HUD Info", "Shows the bot's link status and health on your screen.", Category.RENDER);
        this.enabled = true;
    }
}
