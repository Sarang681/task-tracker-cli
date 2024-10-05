package com.example;

import com.example.service.TaskTrackerService;

public class Main {
    public static void main(String[] args) {
        TaskTrackerService taskTrackerService = new TaskTrackerService();

        taskTrackerService.performTaskFromCli(args);
    }
}