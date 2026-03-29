#include <iostream>
#include <string>
#include <map>

class UserService {
private:
    std::map<int, std::string> users;

public:
    UserService() = default;

    void createUser(int id, const std::string& name) {
        users[id] = name;
    }

    std::string getUser(int id) {
        return users[id];
    }

    void deleteUser(int id) {
        users.erase(id);
    }
};

int main() {
    UserService service;
    service.createUser(1, "Alice");
    std::cout << service.getUser(1) << std::endl;
    return 0;
}
