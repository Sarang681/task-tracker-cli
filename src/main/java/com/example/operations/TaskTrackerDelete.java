package com.example.operations;

public class TaskTrackerDelete implements TaskTrackerOperation {

  @Override
  public void performOperation(String[] args) {
    System.out.println("TaskTrackerDelete :: " + args[1]);
  }
}
