package com.example;

import java.util.List;
import java.util.Map;

public class UserService {
    private Map<String, User> users;

    public UserService() {
        this.users = new Map<>();
    }

    public User createUser(String name) {
        return new User(name);
    }

    public User getUser(String id) {
        return users.get(id);
    }

    public void deleteUser(String id) {
        users.remove(id);
    }

    public List<User> listUsers() {
        return List.of();
    }
}

class AdminService extends UserService {
    public void createAdmin(String name) {
        // admin creation
    }
}
