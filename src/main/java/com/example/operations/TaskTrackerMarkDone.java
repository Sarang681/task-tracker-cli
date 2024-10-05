package com.example.operations;

public class TaskTrackerMarkDone implements TaskTrackerOperation {
  @Override
  public void performOperation(String[] args) {
    System.out.println("TaskTrackerMarkDone :: " + args[1]);
  }
}
