package com.afkcompanion.module.modules;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.Protocol;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.network.ClientPlayerEntity;

public class GotoModule extends Module {
    public GotoModule() {
        super("Goto Me", "Sends the bot to your current position once, then turns off.", Category.MOVEMENT);
    }

    @Override
    protected void onEnable() {
        ClientPlayerEntity player = MinecraftClient.getInstance().player;
        if (player != null) {
            AfkCompanionClient.bot.send(Protocol.gotoPos(player.getX(), player.getY(), player.getZ()));
        }
        enabled = false;
    }
}
