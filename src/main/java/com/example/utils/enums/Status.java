package com.example.utils.enums;

import java.util.Arrays;

public enum Status {
    TODO(1, "TODO"), IN_PROGRESS(2, "IN-PROGRESS"), DONE(3, "DONE");

    private int value;
    private String description;

    Status(int value, String description) {
        this.value = value;
        this.description = description;
    }

    public int getValue() {
        return value;
    }

    public String getDescription() {
        return description;
    }

    public static Status getStatus(String text) {
        return Arrays.asList(Status.values()).stream().filter(status -> status.getDescription().equalsIgnoreCase(text)).findFirst().orElse(null);
    }
}
