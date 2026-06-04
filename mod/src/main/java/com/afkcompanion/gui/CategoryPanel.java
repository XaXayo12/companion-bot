package com.afkcompanion.gui;

import java.util.List;

import com.afkcompanion.AfkCompanionClient;
import com.afkcompanion.module.Category;
import com.afkcompanion.module.Module;
import com.afkcompanion.module.setting.Setting;

import net.minecraft.client.gui.DrawContext;

public class CategoryPanel {
    public final Category category;
    public int x;
    public int y;

    public static final int WIDTH = 112;
    private static final int HEADER = 18;
    private static final int ROW = 14;

    private boolean dragging;
    private int grabX;
    private int grabY;

    public CategoryPanel(Category category, int x, int y) {
        this.category = category;
        this.x = x;
        this.y = y;
    }

    private List<Module> modules() {
        return AfkCompanionClient.modules.byCategory(category);
    }

    public void render(DrawContext context, int mouseX, int mouseY) {
        RenderUtil.roundedRect(context, x, y, WIDTH, HEADER, Theme.HEADER);
        RenderUtil.text(context, category.title, x + 8, y + 5, Theme.TEXT);

        int cy = y + HEADER;
        for (Module module : modules()) {
            boolean hover = inside(mouseX, mouseY, x, cy, WIDTH, ROW);
            RenderUtil.roundedRect(context, x, cy, WIDTH, ROW, Theme.PANEL);
            if (hover) {
                RenderUtil.rect(context, x, cy, WIDTH, ROW, Theme.ROW_HOVER);
            }

            int color = module.enabled ? Theme.accent : Theme.TEXT_DIM;
            RenderUtil.text(context, module.name, x + 8, cy + 3, color);
            if (module.locked) {
                RenderUtil.text(context, "ON", x + WIDTH - 20, cy + 3, Theme.accent);
            } else if (module.enabled) {
                RenderUtil.rect(context, x + WIDTH - 10, cy + 5, 4, 4, Theme.accent);
            }
            cy += ROW;

            if (module.expanded) {
                for (Setting setting : module.settings) {
                    RenderUtil.roundedRect(context, x, cy, WIDTH, ROW, Theme.PANEL_DARK);
                    renderSetting(context, setting, cy);
                    cy += ROW;
                }
            }
        }
    }

    private void renderSetting(DrawContext context, Setting setting, int sy) {
        RenderUtil.text(context, setting.name, x + 10, sy + 3, Theme.TEXT_DIM);
        if (setting instanceof Setting.Bool bool) {
            String mark = bool.value ? "+" : "-";
            RenderUtil.text(context, mark, x + WIDTH - 14, sy + 3, bool.value ? Theme.accent : Theme.TEXT_DIM);
        } else if (setting instanceof Setting.Slider slider) {
            String value = format(slider.value);
            RenderUtil.text(context, value, x + WIDTH - 8 - RenderUtil.width(value), sy + 3, Theme.TEXT);
            int trackX = x + 10;
            int trackW = WIDTH - 20;
            int trackY = sy + ROW - 3;
            RenderUtil.rect(context, trackX, trackY, trackW, 1, Theme.TRACK);
            int fill = (int) (trackW * fraction(slider));
            RenderUtil.rect(context, trackX, trackY, fill, 1, Theme.accent);
        } else if (setting instanceof Setting.Mode mode) {
            String value = mode.get();
            RenderUtil.text(context, value, x + WIDTH - 8 - RenderUtil.width(value), sy + 3, Theme.accent);
        }
    }

    public boolean mouseClicked(double mx, double my, int button) {
        if (inside((int) mx, (int) my, x, y, WIDTH, HEADER)) {
            if (button == 0) {
                dragging = true;
                grabX = (int) mx - x;
                grabY = (int) my - y;
            }
            return true;
        }

        int cy = y + HEADER;
        for (Module module : modules()) {
            if (inside((int) mx, (int) my, x, cy, WIDTH, ROW)) {
                if (button == 0) {
                    module.toggle();
                } else if (button == 1) {
                    module.expanded = !module.expanded;
                }
                return true;
            }
            cy += ROW;
            if (module.expanded) {
                for (Setting setting : module.settings) {
                    if (inside((int) mx, (int) my, x, cy, WIDTH, ROW)) {
                        clickSetting(module, setting, mx);
                        return true;
                    }
                    cy += ROW;
                }
            }
        }
        return false;
    }

    private void clickSetting(Module owner, Setting setting, double mx) {
        if (setting instanceof Setting.Bool bool) {
            bool.value = !bool.value;
        } else if (setting instanceof Setting.Mode mode) {
            mode.next();
        } else if (setting instanceof Setting.Slider slider) {
            setSlider(slider, mx);
        }
        owner.onSettingChanged();
    }

    public void mouseDragged(double mx, double my) {
        if (dragging) {
            x = (int) mx - grabX;
            y = (int) my - grabY;
            return;
        }
        int cy = y + HEADER;
        for (Module module : modules()) {
            cy += ROW;
            if (module.expanded) {
                for (Setting setting : module.settings) {
                    if (setting instanceof Setting.Slider slider && inside((int) mx, (int) my, x, cy, WIDTH, ROW)) {
                        setSlider(slider, mx);
                        module.onSettingChanged();
                    }
                    cy += ROW;
                }
            }
        }
    }

    public void mouseReleased() {
        dragging = false;
    }

    private void setSlider(Setting.Slider slider, double mx) {
        int trackX = x + 10;
        int trackW = WIDTH - 20;
        double f = clamp((mx - trackX) / trackW, 0, 1);
        double raw = slider.min + f * (slider.max - slider.min);
        double stepped = Math.round(raw / slider.step) * slider.step;
        slider.value = clamp(stepped, slider.min, slider.max);
    }

    private double fraction(Setting.Slider slider) {
        return (slider.value - slider.min) / (slider.max - slider.min);
    }

    private static double clamp(double value, double min, double max) {
        return Math.max(min, Math.min(max, value));
    }

    private static String format(double value) {
        if (value == Math.floor(value)) {
            return String.valueOf((int) value);
        }
        return String.format("%.1f", value);
    }

    private static boolean inside(int mx, int my, int x, int y, int width, int height) {
        return mx >= x && mx <= x + width && my >= y && my <= y + height;
    }
}
