package com.afkcompanion.module;

public enum Category {
    COMBAT("Combat"),
    MOVEMENT("Movement"),
    RENDER("Render"),
    PLAYER("Player"),
    MISC("Misc");

    public final String title;

    Category(String title) {
        this.title = title;
    }
}
