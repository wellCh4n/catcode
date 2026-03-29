<?php

class UserService {
    private array $users = [];

    public function createUser(string $name): array {
        $id = count($this->users) + 1;
        $user = ['id' => $id, 'name' => $name];
        $this->users[$id] = $user;
        return $user;
    }

    public function getUser(int $id): ?array {
        return $this->users[$id] ?? null;
    }

    public function deleteUser(int $id): void {
        unset($this->users[$id]);
    }
}

class AdminService extends UserService {
    public function createAdmin(string $name, string $role): array {
        $user = $this->createUser($name);
        $user['role'] = $role;
        return $user;
    }
}

function helper_function(): string {
    return 'helper';
}
