package com.example.service;

import com.example.operations.TaskTrackerDelete;
import com.example.operations.TaskTrackerList;
import com.example.operations.TaskTrackerMarkDone;
import com.example.operations.TaskTrackerMarkInProgress;
import com.example.utils.enums.Command;
import com.example.operations.TaskTrackerAddition;
import com.example.operations.TaskTrackerOperation;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;

public class TaskTrackerService {

  public void performTaskFromCli(String[] args) {

    if (args.length == 0) {
      System.out.println("Please provide a command");
      return;
    }

    String operation = args[0];
    Command command = Command.getCommandFromString(operation);

    if (command == null) {
      System.out.println("Unknown command: " + operation);
      return;
    }

    String fileContents = fetchFileContentsFromJson();

    TaskTrackerOperation taskTrackerOperation;

    switch (command) {
      case ADD:
        {
          taskTrackerOperation = new TaskTrackerAddition();
          taskTrackerOperation.performOperation(args, fileContents);
          break;
        }
      case MARK_IN_PROGRESS:
        {
          taskTrackerOperation = new TaskTrackerMarkInProgress();
          taskTrackerOperation.performOperation(args, fileContents);
          break;
        }
      case MARK_DONE:
        {
          taskTrackerOperation = new TaskTrackerMarkDone();
          taskTrackerOperation.performOperation(args, fileContents);
          break;
        }
      case DELETE:
        {
          taskTrackerOperation = new TaskTrackerDelete();
          taskTrackerOperation.performOperation(args, fileContents);
          break;
        }
      case LIST:
        {
          taskTrackerOperation = new TaskTrackerList();
          taskTrackerOperation.performOperation(args, fileContents);
          break;
        }
      default:
        {
          System.out.println("Unknown command: " + operation);
        }
    }
  }

  private String fetchFileContentsFromJson() {
    String fileContents = "";

    try {
      Path path = Paths.get("src", "main", "resources", "tasktracker.json");
      File newFile = new File(path.toUri());
      if (newFile.createNewFile()) {
        try (OutputStream outputStream = new FileOutputStream(newFile)) {
          outputStream.write("[]".getBytes(StandardCharsets.UTF_8));
        }
      }

      try (InputStream inputStream = new FileInputStream(newFile)) {
        fileContents = new String(inputStream.readAllBytes(), StandardCharsets.UTF_8);
      }

    } catch (IOException e) {
      System.out.println("TaskTracker file creation failed");
    }
    return fileContents;
  }
}
