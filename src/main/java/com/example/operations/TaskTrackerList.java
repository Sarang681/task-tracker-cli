package com.example.operations;

public class TaskTrackerList implements TaskTrackerOperation {
  @Override
  public void performOperation(String[] args) {
    System.out.println("TaskTrackerList :: " + args[1]);
  }
}
