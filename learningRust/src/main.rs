trait Animal {
    fn speak(&self);
}
struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
fn make_animal_speak(animal: &dyn Animal){
    animal.speak();
}
fn main() {
    let dog = Dog;
    let cat = Cat;

    make_animal_speak(&dog);
    make_animal_speak(&cat);
}
