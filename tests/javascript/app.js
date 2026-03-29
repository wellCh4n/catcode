const fs = require('fs');
const path = require('path');

class UserService {
    constructor() {
        this.users = new Map();
    }

    createUser(name) {
        const user = { id: this.users.size + 1, name };
        this.users.set(user.id, user);
        return user;
    }

    getUser(id) {
        return this.users.get(id);
    }

    deleteUser(id) {
        this.users.delete(id);
    }
}

function helperFunction() {
    return 'helper';
}

module.exports = { UserService, helperFunction };
