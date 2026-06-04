package com.afkcompanion.net;

/**
 * Live state of a single bot account, updated from the bot's telemetry. Each
 * connected account has its own instance, so the GUI can show every bot's status
 * independently.
 */
public class BotState {
    public volatile String id = "";
    public volatile String status = "offline";
    public volatile float health;
    public volatile int food;
    public volatile double x;
    public volatile double y;
    public volatile double z;
    public volatile String mode = "idle";
    public volatile String lastLog = "";

    public BotState(String id) {
        this.id = id;
    }
}
