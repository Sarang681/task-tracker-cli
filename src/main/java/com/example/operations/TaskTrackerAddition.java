package com.example.operations;

import com.example.pojo.Task;
import com.example.utils.TaskUtils;
import com.example.utils.enums.Status;
import com.google.gson.Gson;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.sql.Timestamp;
import java.time.Instant;
import java.util.List;

public class TaskTrackerAddition implements TaskTrackerOperation {

  @Override
  public void performOperation(String[] args, String fileContents) {

    if (args.length != 2) {
      System.out.println("Invalid number of arguments");
      return;
    }

    if (args[1] == null || args[1].trim().isEmpty()) {
      System.out.println("Please enter the task description to add");
      return;
    }

    Gson gson = new Gson();

    List<Task> tasks = TaskUtils.fetchAllTasksFromFileContents(fileContents);

    Task newTask = new Task();
    if (tasks.isEmpty()) {
      newTask.setId(0);
    } else {
      Task lastTask = tasks.get(tasks.size() - 1);
      newTask.setId(lastTask.getId() + 1);
    }
    newTask.setDescription(args[1]);
    newTask.setStatus(Status.TODO);
    newTask.setCreatedAt(Timestamp.from(Instant.now()));

    tasks.add(newTask);
    String newTasksJson = gson.toJson(tasks);

    Path filePath = Paths.get("src", "main", "resources", "tasktracker.json");
    File file = new File(filePath.toUri());
    try (OutputStream outputStream = new FileOutputStream(file)) {
      outputStream.write(newTasksJson.getBytes(StandardCharsets.UTF_8));
      System.out.println("New Task Added Successfully. (ID :: " + newTask.getId() + ")");
    } catch (IOException e) {
      System.out.println("Error while adding the new task");
    }
  }
}
