class Animal {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    getName(): string {
        return this.name;
    }

    getAge(): number {
        return this.age;
    }

    static of(name: string, age: number): Animal {
        return new Animal(name, age);
    }
}

class Dog extends Animal {
    breed: string;

    constructor(name: string, age: number, breed: string) {
        super(name, age);
        this.breed = breed;
    }

    static of(name: string, age: number, breed: string): Dog {
        return new Dog(name, age, breed);
    }

    bark(): string {
        return `${this.name} says: Woof!`;
    }
}
