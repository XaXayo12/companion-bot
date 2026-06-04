package com.afkcompanion.net;

import com.google.gson.JsonObject;

public final class Protocol {
    private Protocol() {
    }

    private static JsonObject cmd(String name) {
        JsonObject object = new JsonObject();
        object.addProperty("cmd", name);
        return object;
    }

    public static String follow(String target) {
        JsonObject object = cmd("follow");
        if (target != null && !target.isEmpty()) {
            object.addProperty("target", target);
        }
        return object.toString();
    }

    public static String protect(String target) {
        JsonObject object = cmd("protect");
        if (target != null && !target.isEmpty()) {
            object.addProperty("target", target);
        }
        return object.toString();
    }

    public static String stop() {
        return cmd("stop").toString();
    }

    public static String killAura(boolean on, double range, boolean stayStill) {
        JsonObject object = cmd("killaura");
        object.addProperty("on", on);
        object.addProperty("range", range);
        object.addProperty("stay_still", stayStill);
        return object.toString();
    }

    public static String holdPosition(boolean on) {
        JsonObject object = cmd("hold_position");
        object.addProperty("on", on);
        return object.toString();
    }

    public static String mineStart() {
        return cmd("mine_start").toString();
    }

    public static String pull(String arg) {
        JsonObject object = cmd("pull");
        object.addProperty("arg", arg);
        return object.toString();
    }

    public static String autoTotem(boolean on) {
        JsonObject object = cmd("auto_totem");
        object.addProperty("on", on);
        return object.toString();
    }

    public static String autoArmor(boolean on) {
        JsonObject object = cmd("auto_armor");
        object.addProperty("on", on);
        return object.toString();
    }

    public static String autoDisconnect(boolean on, double health) {
        JsonObject object = cmd("auto_disconnect");
        object.addProperty("on", on);
        object.addProperty("health", health);
        return object.toString();
    }

    public static String gotoPos(double x, double y, double z) {
        JsonObject object = cmd("goto");
        object.addProperty("x", x);
        object.addProperty("y", y);
        object.addProperty("z", z);
        return object.toString();
    }

    public static String followDistance(double distance) {
        JsonObject object = cmd("follow_distance");
        object.addProperty("distance", distance);
        return object.toString();
    }

    public static String sprint(boolean on) {
        JsonObject object = cmd("sprint");
        object.addProperty("on", on);
        return object.toString();
    }

    public static String sneak(boolean on) {
        JsonObject object = cmd("sneak");
        object.addProperty("on", on);
        return object.toString();
    }

    public static String deposit() {
        return cmd("deposit").toString();
    }

    public static String dropTrash() {
        return cmd("drop_trash").toString();
    }

    /** Tell the bot to eat whenever its food level drops to/below {@code food}. */
    public static String eatThreshold(int food) {
        JsonObject object = cmd("eat_threshold");
        object.addProperty("food", food);
        return object.toString();
    }

    /** Make the bot say a line in the in-game chat. */
    public static String say(String message) {
        JsonObject object = cmd("say");
        object.addProperty("text", message == null ? "" : message);
        return object.toString();
    }

    /** Tell the bot your in-game name so only you can command it by whisper. */
    public static String setOwner(String name) {
        JsonObject object = cmd("set_owner");
        object.addProperty("owner", name == null ? "" : name);
        return object.toString();
    }
}
