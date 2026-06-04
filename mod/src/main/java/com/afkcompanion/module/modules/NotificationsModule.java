package com.afkcompanion.module.modules;

import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;

public class NotificationsModule extends Module {
    public NotificationsModule() {
        super("Notifications", "Print bot status changes and warnings (e.g. \"I can't see you\") in your chat.", Category.MISC);
        this.enabled = true;
    }
}
