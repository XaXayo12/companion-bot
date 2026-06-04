package com.afkcompanion.module.setting;

import java.util.List;

public abstract class Setting {
    public final String name;

    protected Setting(String name) {
        this.name = name;
    }

    public static class Bool extends Setting {
        public boolean value;

        public Bool(String name, boolean value) {
            super(name);
            this.value = value;
        }
    }

    public static class Slider extends Setting {
        public double value;
        public final double min;
        public final double max;
        public final double step;

        public Slider(String name, double value, double min, double max, double step) {
            super(name);
            this.value = value;
            this.min = min;
            this.max = max;
            this.step = step;
        }
    }

    public static class Mode extends Setting {
        public final List<String> options;
        public int index;

        public Mode(String name, List<String> options, int index) {
            super(name);
            this.options = options;
            this.index = index;
        }

        public String get() {
            return options.get(index);
        }

        public void next() {
            index = (index + 1) % options.size();
        }
    }
}
