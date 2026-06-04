package com.afkcompanion.gui;

import java.util.ArrayList;
import java.util.List;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.net.BotLink;
import com.afkcompanion.net.BotState;
import com.afkcompanion.net.Protocol;

import net.minecraft.client.gui.Click;
import net.minecraft.client.gui.DrawContext;
import net.minecraft.client.gui.screen.Screen;
import net.minecraft.client.gui.widget.ButtonWidget;
import net.minecraft.client.gui.widget.TextFieldWidget;
import net.minecraft.text.Text;

public class ClickGuiScreen extends Screen {
    private static final List<CategoryPanel> PANELS = new ArrayList<>();
    private static final int BAR_Y = 4;
    private static final int CHIP_HEIGHT = 11;

    /** Bottom chat bar: type a line and the selected bot says it in game. */
    private TextFieldWidget chatInput;

    public ClickGuiScreen() {
        super(Text.literal("AFK Companion"));
        if (PANELS.isEmpty()) {
            int x = 16;
            for (Category category : Category.values()) {
                PANELS.add(new CategoryPanel(category, x, 30));
                x += CategoryPanel.WIDTH + 8;
            }
        }
    }

    @Override
    protected void init() {
        // The say bar: a text field plus a Send button, near the bottom. What you
        // type is sent to the selected bot, which says it in the in-game chat.
        int barY = this.height - 42;
        chatInput = new TextFieldWidget(this.textRenderer, 40, barY, 220, 14, Text.literal("say"));
        chatInput.setMaxLength(256);
        addDrawableChild(chatInput);
        addDrawableChild(ButtonWidget.builder(Text.literal("Send"), b -> sendSay())
                .dimensions(264, barY, 42, 14)
                .build());
    }

    private void sendSay() {
        if (chatInput == null) {
            return;
        }
        String text = chatInput.getText().trim();
        if (!text.isEmpty()) {
            AfkCompanionClient.bot.send(Protocol.say(text));
            chatInput.setText("");
        }
    }

    @Override
    public void render(DrawContext context, int mouseX, int mouseY, float delta) {
        if (backgroundEnabled()) {
            context.fill(0, 0, this.width, this.height, Theme.BG);
        }
        super.render(context, mouseX, mouseY, delta);

        renderAccountBar(context);

        for (CategoryPanel panel : PANELS) {
            panel.render(context, mouseX, mouseY);
        }

        BotLink link = AfkCompanionClient.bot;
        BotState state = link.selectedState();
        String line = link.linked()
                ? "Bot link: connected (" + state.status + ")"
                : "Bot link: offline";
        RenderUtil.text(context, line, 6, this.height - 12, link.linked() ? Theme.accent : Theme.TEXT_DIM);
        RenderUtil.text(context,
                "Click an account above to control it.  In game, whisper /msg <bot> !follow  !come  !stop  !pull.",
                6, this.height - 24, Theme.TEXT_DIM);

        // Label for the say bar (the field + Send button are drawn as widgets).
        RenderUtil.text(context, "Say:", 16, this.height - 39, Theme.accent);
    }

    // The account selector: one chip for ALL plus one per connected bot. The
    // selected chip is highlighted; clicking a chip changes which bot every
    // command is sent to.
    private void renderAccountBar(DrawContext context) {
        BotLink link = AfkCompanionClient.bot;
        if (!link.linked()) {
            RenderUtil.text(context, "Waiting for the bot...", 16, BAR_Y + 2, Theme.TEXT_DIM);
            return;
        }
        for (Chip chip : accountChips()) {
            boolean selected = chip.id.equals(link.selected());
            RenderUtil.rect(context, chip.x, chip.y, chip.w, chip.h, selected ? Theme.accent : Theme.PANEL);
            RenderUtil.text(context, chip.label, chip.x + 4, chip.y + 2, selected ? Theme.BG : Theme.TEXT);
            // A small dot shows whether that specific account is online in game.
            if (!chip.id.equals(BotLink.ALL)) {
                BotState st = link.stateOf(chip.id);
                boolean online = st != null && "online".equals(st.status);
                RenderUtil.rect(context, chip.x + chip.w - 4, chip.y + 2, 2, 2,
                        online ? Theme.ONLINE : Theme.TEXT_DIM);
            }
        }
    }

    private List<Chip> accountChips() {
        BotLink link = AfkCompanionClient.bot;
        List<Chip> chips = new ArrayList<>();
        List<String> ids = new ArrayList<>();
        ids.add(BotLink.ALL);
        ids.addAll(link.botIds());
        int x = 16;
        for (String id : ids) {
            boolean all = id.equals(BotLink.ALL);
            String label = all ? "ALL" : id;
            // Leave a little extra room on bot chips for the online status dot.
            int w = RenderUtil.width(label) + (all ? 8 : 12);
            chips.add(new Chip(id, label, x, BAR_Y, w, CHIP_HEIGHT));
            x += w + 4;
        }
        return chips;
    }

    @Override
    public boolean mouseClicked(Click click, boolean doubled) {
        BotLink link = AfkCompanionClient.bot;
        if (link.linked()) {
            for (Chip chip : accountChips()) {
                if (chip.contains(click.x(), click.y())) {
                    link.select(chip.id);
                    return true;
                }
            }
        }
        for (CategoryPanel panel : PANELS) {
            if (panel.mouseClicked(click.x(), click.y(), click.button())) {
                return true;
            }
        }
        return super.mouseClicked(click, doubled);
    }

    @Override
    public boolean mouseDragged(Click click, double dx, double dy) {
        for (CategoryPanel panel : PANELS) {
            panel.mouseDragged(click.x(), click.y());
        }
        return super.mouseDragged(click, dx, dy);
    }

    @Override
    public boolean mouseReleased(Click click) {
        for (CategoryPanel panel : PANELS) {
            panel.mouseReleased();
        }
        return super.mouseReleased(click);
    }

    @Override
    public boolean shouldPause() {
        return false;
    }

    private boolean backgroundEnabled() {
        Module module = AfkCompanionClient.modules.byName("Background");
        return module == null || module.enabled;
    }

    private record Chip(String id, String label, int x, int y, int w, int h) {
        boolean contains(double mx, double my) {
            return mx >= x && mx <= x + w && my >= y && my <= y + h;
        }
    }
}
