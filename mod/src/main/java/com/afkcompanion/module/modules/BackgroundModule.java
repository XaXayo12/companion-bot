package com.afkcompanion.module.modules;

import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;

public class BackgroundModule extends Module {
    public BackgroundModule() {
        super("Background", "Dim the screen behind the menu.", Category.RENDER);
        this.enabled = true;
    }
}
