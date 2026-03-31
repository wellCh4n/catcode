class Animal:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def get_name(self):
        return self.name

    def get_age(self):
        return self.age

    @staticmethod
    def of(name, age):
        return Animal(name, age)


class Dog(Animal):
    def __init__(self, name, age, breed):
        super().__init__(name, age)
        self.breed = breed

    @staticmethod
    def of(name, age, breed):
        return Dog(name, age, breed)

    def get_breed(self):
        return self.breed

    def bark(self):
        return f"{self.name} says: Woof!"
