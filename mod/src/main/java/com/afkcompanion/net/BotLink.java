package com.afkcompanion.net;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.WebSocket;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.CompletionStage;

import com.afkcompanion.AfkCompanion;
import com.google.gson.Gson;
import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

/**
 * Single WebSocket link to the bot process. The process may drive several
 * accounts at once, so this link keeps one {@link BotState} per account, tracks
 * which account the GUI currently controls, and tags every outgoing command with
 * that account's id. The link itself ({@link #linked}) is separate from any
 * single bot's status.
 */
public class BotLink {
    /** Wire value that targets every connected account at once. */
    public static final String ALL = "all";

    private final String host;
    private final int port;
    private final Gson gson = new Gson();

    private final Map<String, BotState> bots = new ConcurrentHashMap<>();
    /**
     * Human-readable notices (status changes, errors) waiting to be shown in the
     * player's chat by the Notifications module. Filled on the WebSocket thread,
     * drained on the client thread.
     */
    private final Queue<Notice> notifications = new ConcurrentLinkedQueue<>();
    private volatile String selected = ALL;
    private volatile boolean linked;

    private volatile WebSocket socket;
    private volatile boolean connecting;
    private long nextRetryAt;

    public BotLink(String host, int port) {
        this.host = host;
        this.port = port;
    }

    public boolean linked() {
        return linked;
    }

    /** Sorted ids of every account the bot has announced. */
    public List<String> botIds() {
        List<String> ids = new ArrayList<>(bots.keySet());
        ids.sort(String::compareTo);
        return ids;
    }

    /** The account the GUI currently controls, or {@link #ALL}. */
    public String selected() {
        return selected;
    }

    public void select(String id) {
        if (ALL.equals(id) || bots.containsKey(id)) {
            selected = id;
        }
    }

    /** Cycle the selection through {@code ALL} and every account, in order. */
    public void selectNext() {
        List<String> order = new ArrayList<>();
        order.add(ALL);
        order.addAll(botIds());
        int index = order.indexOf(selected);
        selected = order.get((index + 1) % order.size());
    }

    /** State for a specific account id, or {@code null} if it is unknown. */
    public BotState stateOf(String id) {
        return bots.get(id);
    }

    /** State of the selected account, or a synthetic summary when ALL is active. */
    public BotState selectedState() {
        if (!ALL.equals(selected)) {
            BotState state = bots.get(selected);
            if (state != null) {
                return state;
            }
        }
        // ALL selected, or the selected account vanished: show the first account
        // if there is one, otherwise an empty placeholder.
        List<String> ids = botIds();
        if (!ids.isEmpty()) {
            return bots.get(ids.get(0));
        }
        return new BotState("");
    }

    /** Take the next queued chat notice, or {@code null} if there are none. */
    public Notice pollNotice() {
        return notifications.poll();
    }

    /** A one-line message for the player's chat. {@code error} = show it in red. */
    public record Notice(String text, boolean error) {
    }

    public void connect() {
        if (connecting || linked) {
            return;
        }
        connecting = true;
        try {
            HttpClient.newHttpClient()
                    .newWebSocketBuilder()
                    .buildAsync(URI.create("ws://" + host + ":" + port), new Handler())
                    .whenComplete((webSocket, error) -> {
                        connecting = false;
                        if (error != null) {
                            linked = false;
                            scheduleRetry();
                        } else {
                            socket = webSocket;
                            linked = true;
                            AfkCompanion.LOG.info("Linked to bot at {}:{}", host, port);
                        }
                    });
        } catch (Throwable t) {
            connecting = false;
            scheduleRetry();
        }
    }

    public void tick() {
        if (!linked && !connecting && System.currentTimeMillis() >= nextRetryAt) {
            connect();
        }
    }

    /**
     * Send a command JSON, tagging it with the currently selected account so the
     * bot routes it to exactly that bot (or every bot when ALL is selected).
     */
    public void send(String json) {
        WebSocket webSocket = socket;
        if (webSocket == null || !linked) {
            return;
        }
        try {
            JsonObject object = gson.fromJson(json, JsonObject.class);
            if (object == null) {
                return;
            }
            object.addProperty("bot", selected);
            webSocket.sendText(object.toString(), true);
        } catch (Throwable t) {
            AfkCompanion.LOG.warn("Failed to send command to bot", t);
        }
    }

    private void scheduleRetry() {
        nextRetryAt = System.currentTimeMillis() + 3000;
    }

    private BotState bot(String id) {
        return bots.computeIfAbsent(id, BotState::new);
    }

    private void handle(String text) {
        try {
            JsonObject in = gson.fromJson(text, JsonObject.class);
            if (in == null || !in.has("event")) {
                return;
            }
            String event = in.get("event").getAsString();
            String id = string(in, "bot", "");
            switch (event) {
                case "roster" -> updateRoster(in);
                case "status" -> {
                    // A status with a bot id is that account's status; without one
                    // it is the link itself (e.g. "connected") and is ignored here.
                    if (!id.isEmpty()) {
                        String newStatus = string(in, "status", "unknown");
                        BotState st = bot(id);
                        if (!newStatus.equals(st.status)) {
                            st.status = newStatus;
                            notifications.add(new Notice("Bot " + id + " is now " + newStatus, false));
                        }
                    }
                }
                case "error" -> {
                    // A problem the bot wants the player to see (e.g. "I can't see
                    // you" when you are on a different server). Always queued; the
                    // Notifications module decides whether to print it.
                    String message = string(in, "text", "");
                    if (!message.isEmpty()) {
                        String prefix = id.isEmpty() ? "" : "[Bot " + id + "] ";
                        notifications.add(new Notice(prefix + message, true));
                    }
                }
                case "telemetry" -> {
                    if (!id.isEmpty()) {
                        BotState state = bot(id);
                        state.health = (float) number(in, "health");
                        state.food = (int) number(in, "food");
                        state.x = number(in, "x");
                        state.y = number(in, "y");
                        state.z = number(in, "z");
                        state.mode = string(in, "mode", "idle");
                    }
                }
                case "log" -> {
                    if (!id.isEmpty()) {
                        bot(id).lastLog = string(in, "text", "");
                    }
                }
                default -> {
                }
            }
        } catch (Throwable ignored) {
        }
    }

    private void updateRoster(JsonObject in) {
        if (!in.has("bots") || !in.get("bots").isJsonArray()) {
            return;
        }
        JsonArray array = in.getAsJsonArray("bots");
        for (int i = 0; i < array.size(); i++) {
            bot(array.get(i).getAsString());
        }
        // If nothing is meaningfully selected yet, focus the first account.
        if (ALL.equals(selected) && bots.size() == 1) {
            selected = botIds().get(0);
        }
    }

    private static String string(JsonObject object, String key, String fallback) {
        return object.has(key) && !object.get(key).isJsonNull() ? object.get(key).getAsString() : fallback;
    }

    private static double number(JsonObject object, String key) {
        return object.has(key) && !object.get(key).isJsonNull() ? object.get(key).getAsDouble() : 0;
    }

    private class Handler implements WebSocket.Listener {
        private final StringBuilder buffer = new StringBuilder();

        @Override
        public void onOpen(WebSocket webSocket) {
            linked = true;
            webSocket.request(1);
        }

        @Override
        public CompletionStage<?> onText(WebSocket webSocket, CharSequence data, boolean last) {
            buffer.append(data);
            if (last) {
                handle(buffer.toString());
                buffer.setLength(0);
            }
            webSocket.request(1);
            return null;
        }

        @Override
        public CompletionStage<?> onClose(WebSocket webSocket, int statusCode, String reason) {
            linked = false;
            socket = null;
            scheduleRetry();
            return null;
        }

        @Override
        public void onError(WebSocket webSocket, Throwable error) {
            linked = false;
            socket = null;
            scheduleRetry();
        }
    }
}
