class Animal {
    name;
    age;

    constructor(name, age) {
        this.name = name;
        this.age = age;
    }

    getName() {
        return this.name;
    }

    getAge() {
        return this.age;
    }

    static of(name, age) {
        return new Animal(name, age);
    }
}

class Dog extends Animal {
    breed;

    constructor(name, age, breed) {
        super(name, age);
        this.breed = breed;
    }

    static of(name, age, breed) {
        return new Dog(name, age, breed);
    }

    bark() {
        return `${this.name} says: Woof!`;
    }
}
