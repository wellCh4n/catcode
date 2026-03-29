class UserService:
    def create_user(self, name: str) -> User:
        pass

    def get_user(self, user_id: int) -> User:
        pass

    def delete_user(self, user_id: int) -> None:
        pass


class OrderService:
    def create_order(self, user_id: int, items: List[str]) -> Order:
        pass

    def get_order(self, order_id: int) -> Order:
        pass


def helper_function():
    pass
