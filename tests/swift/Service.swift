import Foundation

class UserService {
    private var users: [Int: User] = [:]

    func createUser(name: String) -> User {
        let id = users.count + 1
        let user = User(id: id, name: name)
        users[id] = user
        return user
    }

    func getUser(id: Int) -> User? {
        return users[id]
    }

    func deleteUser(id: Int) {
        users.removeValue(forKey: id)
    }
}

struct User {
    let id: Int
    let name: String
}
