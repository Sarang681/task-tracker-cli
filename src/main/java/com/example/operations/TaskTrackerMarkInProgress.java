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
import java.util.List;

public class TaskTrackerMarkInProgress implements TaskTrackerOperation {

  @Override
  public void performOperation(String[] args, String fileContents) {
    if (args.length != 2) {
      System.out.println("Please enter two arguments");
      return;
    }

    int taskId = -1;

    try {
      taskId = Integer.parseInt(args[1]);
    } catch (NumberFormatException e) {
      System.out.println("Please enter a valid task id" + e.getMessage());
      return;
    }

    if (taskId == -1) {
      System.out.println("Please enter a valid task id");
      return;
    }

    List<Task> tasks = TaskUtils.fetchAllTasksFromFileContents(fileContents);

    if (tasks.isEmpty()) {
      System.out.println("No tasks found");
      return;
    }

    int finalTaskId = taskId;
    Task taskToUpdate =
            tasks.stream().filter(task -> task.getId() == finalTaskId).findFirst().orElse(null);

    if (taskToUpdate != null) {
      taskToUpdate.setStatus(Status.IN_PROGRESS);
    } else {
      System.out.println("No task found with id " + finalTaskId);
      return;
    }

    String newTaskList = new Gson().toJson(tasks);

    Path filePath = Paths.get("src", "main", "resources", "tasktracker.json");
    File file = new File(filePath.toUri());
    try (OutputStream outputStream = new FileOutputStream(file)) {
      outputStream.write(newTaskList.getBytes(StandardCharsets.UTF_8));
    } catch (IOException e) {
      System.out.println("Error while adding the new task");
    }
  }
}
