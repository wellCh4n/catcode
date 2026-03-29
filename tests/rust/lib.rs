use std::collections::HashMap;

pub struct UserService {
    users: HashMap<String, User>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn create_user(&mut self, name: String) -> User {
        User { name }
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn delete_user(&mut self, id: &str) {
        self.users.remove(id);
    }
}

pub struct User {
    pub name: String,
}

pub fn helper_function() -> String {
    String::from("helper")
}
