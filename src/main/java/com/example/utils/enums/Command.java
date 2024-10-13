package com.example.utils.enums;

import java.util.Arrays;

public enum Command {
    ADD(1, "add"),
    MARK_IN_PROGRESS(2, "mark-in-progress"),
    MARK_DONE(3, "mark-done"),
    DELETE(4, "delete"),
    LIST(5, "list"),
    UPDATE(6, "update");

    private Integer index;
    private String command;

    Command(Integer index, String command) {
        this.index = index;
        this.command = command;
    }

    public Integer getIndex() {
        return this.index;
    }

    public String getCommand() {
        return this.command;
    }

    public static Command getCommandFromString(String text) {
        return Arrays.stream(Command.values()).filter(command -> command.getCommand().equalsIgnoreCase(text))
                .findFirst().orElse(null);
    }

}

