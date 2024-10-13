package com.example.operations;

import com.example.pojo.Task;
import com.example.utils.TaskUtils;
import com.example.utils.enums.Status;

import java.util.List;

public class TaskTrackerList implements TaskTrackerOperation {
  @Override
  public void performOperation(String[] args, String fileContents) {

    if (args.length < 1) {
      System.out.println("Please enter atleast 2 arguments");
    } else if (args.length == 1) {
      List<Task> tasks = TaskUtils.fetchAllTasksFromFileContents(fileContents);
      tasks.forEach(System.out::println);
    } else if (args.length == 2) {
      Status status = Status.getStatus(args[1]);
      if (status != null) {
        List<Task> tasks = TaskUtils.fetchAllTasksFromFileContents(fileContents);
        tasks.stream().filter(task -> task.getStatus() == status).forEach(System.out::println);
      } else {
        System.out.println("Invalid status");
      }
    }

  }
}
