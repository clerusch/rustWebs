fn main(){

    struct Person {
        name: String,
        age: u32,
    }
    impl Person {
        fn new(name: String, age: u32) -> Self {
            Self { name, age }
        }
    
        fn greet(&self) {
            println!("Hallo, ich bin {} und bin {} Jahre alt", self.name, self.age);
        }
    }
    
    let henri = Person::new("Henri".to_string(), 24);
    let clemens = Person{
        name: "clemens".to_string(), 
        age: 26,
    };
    println!("Ich bin {1} und {0} Jahre alt", clemens.age, clemens.name);
    henri.greet();
}