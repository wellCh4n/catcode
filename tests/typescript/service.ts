export class UserService {
    private users: Map<number, User>;

    constructor() {
        this.users = new Map();
    }

    createUser(name: string): User {
        const id = this.users.size + 1;
        const user = { id, name };
        this.users.set(id, user);
        return user;
    }

    getUser(id: number): User | undefined {
        return this.users.get(id);
    }

    deleteUser(id: number): void {
        this.users.delete(id);
    }
}

interface User {
    id: number;
    name: string;
}
