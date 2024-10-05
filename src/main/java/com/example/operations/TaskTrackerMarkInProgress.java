package com.example.operations;

public class TaskTrackerMarkInProgress implements TaskTrackerOperation {

  @Override
  public void performOperation(String[] args) {
    System.out.println("TaskTrackerMarkInProgress ::" + args[1]);
  }
}
