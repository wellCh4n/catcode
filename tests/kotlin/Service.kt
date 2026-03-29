package com.example

import java.util.HashMap

class UserService {
    private val users = HashMap<String, User>()

    fun createUser(name: String): User {
        val id = users.size + 1
        val user = User(id, name)
        users[id.toString()] = user
        return user
    }

    fun getUser(id: String): User? {
        return users[id]
    }

    fun deleteUser(id: String) {
        users.remove(id)
    }
}

class AdminService : UserService() {
    fun createAdmin(name: String, role: String): User {
        return createUser(name)
    }
}

data class User(val id: Int, val name: String)
