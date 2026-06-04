package com.afkcompanion.module;

import java.util.ArrayList;
import java.util.List;

import com.afkcompanion.module.modules.AutoArmorModule;
import com.afkcompanion.module.modules.AutoDisconnectModule;
import com.afkcompanion.module.modules.AutoEatModule;
import com.afkcompanion.module.modules.AutoPullModule;
import com.afkcompanion.module.modules.AutoTotemModule;
import com.afkcompanion.module.modules.BackgroundModule;
import com.afkcompanion.module.modules.DeathAlertModule;
import com.afkcompanion.module.modules.DepositModule;
import com.afkcompanion.module.modules.DropTrashModule;
import com.afkcompanion.module.modules.FollowModule;
import com.afkcompanion.module.modules.GotoModule;
import com.afkcompanion.module.modules.HoldPositionModule;
import com.afkcompanion.module.modules.HudInfoModule;
import com.afkcompanion.module.modules.KillAuraModule;
import com.afkcompanion.module.modules.MineModule;
import com.afkcompanion.module.modules.NotificationsModule;
import com.afkcompanion.module.modules.ProtectModule;
import com.afkcompanion.module.modules.SneakModule;
import com.afkcompanion.module.modules.SprintModule;
import com.afkcompanion.module.modules.TracerModule;

public class ModuleManager {
    private final List<Module> modules = new ArrayList<>();

    public ModuleManager() {
        register(new KillAuraModule());
        register(new ProtectModule());
        register(new AutoTotemModule());
        register(new AutoDisconnectModule());
        register(new FollowModule());
        register(new HoldPositionModule());
        register(new GotoModule());
        register(new SprintModule());
        register(new SneakModule());
        register(new HudInfoModule());
        register(new TracerModule());
        register(new BackgroundModule());
        register(new AutoEatModule());
        register(new AutoArmorModule());
        register(new AutoPullModule());
        register(new DepositModule());
        register(new DeathAlertModule());
        register(new NotificationsModule());
        register(new MineModule());
        register(new DropTrashModule());
    }

    private void register(Module module) {
        modules.add(module);
    }

    public List<Module> all() {
        return modules;
    }

    public List<Module> byCategory(Category category) {
        List<Module> result = new ArrayList<>();
        for (Module module : modules) {
            if (module.category == category) {
                result.add(module);
            }
        }
        return result;
    }

    public Module byName(String name) {
        for (Module module : modules) {
            if (module.name.equalsIgnoreCase(name)) {
                return module;
            }
        }
        return null;
    }
}
