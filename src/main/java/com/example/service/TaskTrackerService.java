package com.example.service;

import com.example.operations.TaskTrackerDelete;
import com.example.operations.TaskTrackerList;
import com.example.operations.TaskTrackerMarkDone;
import com.example.operations.TaskTrackerMarkInProgress;
import com.example.utils.enums.Command;
import com.example.operations.TaskTrackerAddition;
import com.example.operations.TaskTrackerOperation;

public class TaskTrackerService {

  public void performTaskFromCli(String[] args) {
    String operation = args[0];

    Command command = Command.getCommandFromString(operation);

    if (command == null) {
      System.out.println("Unknown command: " + operation);
      return;
    }

    TaskTrackerOperation taskTrackerOperation;

    switch (command) {
      case ADD:
        {
          taskTrackerOperation = new TaskTrackerAddition();
          taskTrackerOperation.performOperation(args);
          break;
        }
      case MARK_IN_PROGRESS: {
        taskTrackerOperation = new TaskTrackerMarkInProgress();
        taskTrackerOperation.performOperation(args);
        break;
      }
      case MARK_DONE: {
        taskTrackerOperation = new TaskTrackerMarkDone();
        taskTrackerOperation.performOperation(args);
        break;
      }
      case DELETE: {
        taskTrackerOperation = new TaskTrackerDelete();
        taskTrackerOperation.performOperation(args);
        break;
      }
      case LIST: {
        taskTrackerOperation = new TaskTrackerList();
        taskTrackerOperation.performOperation(args);
        break;
      }
      default:
        {
          System.out.println("Unknown command: " + operation);
        }
    }
  }
}
