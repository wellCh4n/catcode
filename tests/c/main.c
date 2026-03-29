#include <stdio.h>
#include <stdlib.h>

struct User {
    int id;
    char* name;
};

struct UserService {
    struct User** users;
    int size;
};

struct UserService* create_user_service() {
    struct UserService* service = malloc(sizeof(struct UserService));
    service->users = NULL;
    service->size = 0;
    return service;
}

struct User* create_user(struct UserService* service, char* name) {
    struct User* user = malloc(sizeof(struct User));
    user->id = service->size + 1;
    user->name = name;
    return user;
}

void delete_user(struct UserService* service, int id) {
    // deletion logic
}
