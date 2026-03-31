public class Animal {

    private String name;
    private int age;

    public Animal(String name, int age) {
        this.name = name;
        this.age = age;
    }

    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }

    public static Animal of(String name, int age) {
        return new Animal(name, age);
    }

    public static class Dog extends Animal {

        private String breed;

        public Dog(String name, int age, String breed) {
            super(name, age);
            this.breed = breed;
        }

        public static Dog of(String name, int age, String breed) {
            return new Dog(name, age, breed);
        }

        public String getBreed() {
            return breed;
        }
    }
}
