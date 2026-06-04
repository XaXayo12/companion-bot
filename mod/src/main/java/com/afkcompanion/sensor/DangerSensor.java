package com.afkcompanion.sensor;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.modules.AutoPullModule;
import com.afkcompanion.net.Protocol;

import net.minecraft.client.MinecraftClient;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Items;

public class DangerSensor {
    public volatile boolean inDanger;
    public volatile boolean noTotem;
    public volatile int totemCount;
    public volatile String reason = "";

    private long lastPullAt;

    public void tick(MinecraftClient client) {
        ClientPlayerEntity player = client.player;
        if (player == null) {
            inDanger = false;
            return;
        }

        float health = player.getHealth();
        totemCount = countTotems(player);
        noTotem = totemCount <= 0;

        boolean danger = false;
        String why = "";
        if (player.isInLava()) {
            danger = true;
            why = "Lava";
        } else if (player.isOnFire()) {
            danger = true;
            why = "Fire";
        } else if (player.fallDistance > 6.0f) {
            danger = true;
            why = "Falling";
        } else if (health <= 6.0f) {
            danger = true;
            why = "Low HP";
        }
        inDanger = danger;
        reason = why;

        maybeAutoPull(health);
    }

    private void maybeAutoPull(float health) {
        Module module = AfkCompanionClient.modules.byName("Auto-Pull");
        if (!(module instanceof AutoPullModule pull) || !pull.enabled) {
            return;
        }
        if (pull.onLowHealth.value
                && health <= pull.healthThreshold.value
                && System.currentTimeMillis() - lastPullAt > 8000) {
            AfkCompanionClient.bot.send(Protocol.pull("health_low"));
            lastPullAt = System.currentTimeMillis();
        }
    }

    private int countTotems(ClientPlayerEntity player) {
        int total = 0;
        for (int i = 0; i < player.getInventory().size(); i++) {
            ItemStack stack = player.getInventory().getStack(i);
            if (stack.isOf(Items.TOTEM_OF_UNDYING)) {
                total += stack.getCount();
            }
        }
        return total;
    }
}
