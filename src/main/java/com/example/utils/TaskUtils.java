package com.example.utils;

import com.example.pojo.Task;
import com.google.gson.Gson;
import org.json.JSONArray;
import org.json.JSONObject;

import java.util.ArrayList;
import java.util.List;

public class TaskUtils {

    // Private constructor to prevent instantiation
    private TaskUtils() {

    }

    public static List<Task> fetchAllTasksFromFileContents(String fileContents) {
        JSONArray tasksJson = new JSONArray(fileContents);
        List<Task> tasks = new ArrayList<>();
        Gson gson = new Gson();
        tasksJson.forEach(
                task -> {
                    JSONObject taskJson = (JSONObject) task;
                    Task newTask = gson.fromJson(taskJson.toString(), Task.class);
                    tasks.add(newTask);
                });

        return tasks;
    }

}
