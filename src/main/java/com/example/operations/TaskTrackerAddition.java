package com.example.operations;

public class TaskTrackerAddition implements TaskTrackerOperation {

  @Override
  public void performOperation(String[] args) {
    System.out.println("TaskTrackerAddition :: " + args[1]);
  }
}
