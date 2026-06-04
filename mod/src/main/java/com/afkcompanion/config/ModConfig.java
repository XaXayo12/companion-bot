package com.afkcompanion.config;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

import com.afkcompanion.AfkCompanion;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import net.fabricmc.loader.api.FabricLoader;

public class ModConfig {
    public String botHost = "127.0.0.1";
    public int botPort = 8765;

    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();

    private static Path file() {
        return FabricLoader.getInstance().getConfigDir().resolve(AfkCompanion.MOD_ID + ".json");
    }

    public static ModConfig load() {
        Path path = file();
        try {
            if (Files.exists(path)) {
                ModConfig loaded = GSON.fromJson(Files.readString(path), ModConfig.class);
                if (loaded != null) {
                    return loaded;
                }
            }
        } catch (Throwable t) {
            AfkCompanion.LOG.warn("Could not read config, using defaults", t);
        }
        ModConfig fresh = new ModConfig();
        fresh.save();
        return fresh;
    }

    public void save() {
        try {
            Files.writeString(file(), GSON.toJson(this));
        } catch (IOException e) {
            AfkCompanion.LOG.warn("Could not save config", e);
        }
    }
}
