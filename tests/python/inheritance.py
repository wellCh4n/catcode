class User:
    def __init__(self, name: str):
        self.name = name

    def get_name(self) -> str:
        return self.name


class AdminUser(User):
    def __init__(self, name: str, role: str):
        super().__init__(name)
        self.role = role

    def get_role(self) -> str:
        return self.role
