package com.example.pojo;

import com.example.utils.enums.Status;

import java.sql.Timestamp;

public class Task {
    private long id;
    private String description;
    private Status status;
    private Timestamp createdAt;
    private Timestamp updatedAt;

    public long getId() {
        return id;
    }

    public void setId(long id) {
        this.id = id;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public Status getStatus() {
        return status;
    }

    public void setStatus(Status status) {
        this.status = status;
    }

    public Timestamp getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Timestamp createdAt) {
        this.createdAt = createdAt;
    }

    public Timestamp getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(Timestamp updatedAt) {
        this.updatedAt = updatedAt;
    }

    @Override
    public String toString() {
        return "{\n" +
                "\tid = " + id + ",\n" +
                "\tdescription = '" + description + '\'' + ",\n" +
                "\tstatus = " + status + ",\n" +
                "\tcreatedAt = " + createdAt + ",\n" +
                "\tupdatedAt = " + updatedAt + ",\n" +
                '}';
    }
}
