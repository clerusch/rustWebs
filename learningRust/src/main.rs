fn main() {
    struct Person {
        name: String,
        age: u32,
    }
    impl Person {
        fn new(name: String, age: u32) -> Self {
            Self { name, age }
        }
    
        fn greet(&self) {
            println!("Hello, my name is {}!", self.name);
        }
    }
    
    let henri = Person::new("Henri".to_string(), 24);
    let clemens = Person{
        name: "clemens".to_string(), 
        age: 26,
    };
    println!("Mein alter ist {0} und mein Name ist {1}", clemens.age, clemens.name);
    henri.greet();
}
