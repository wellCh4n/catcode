using System;
using System.Collections.Generic;

public class UserService
{
    private Dictionary<int, User> users;

    public UserService()
    {
        users = new Dictionary<int, User>();
    }

    public User CreateUser(string name)
    {
        int id = users.Count + 1;
        User user = new User { Id = id, Name = name };
        users[id] = user;
        return user;
    }

    public User GetUser(int id)
    {
        return users[id];
    }

    public void DeleteUser(int id)
    {
        users.Remove(id);
    }
}

public class AdminService : UserService
{
    public User CreateAdmin(string name, string role)
    {
        var user = CreateUser(name);
        return user;
    }
}

public class User
{
    public int Id { get; set; }
    public string Name { get; set; }
}
