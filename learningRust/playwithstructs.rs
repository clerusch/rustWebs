fn main(){

    struct Person {
        name: String,
        age: u32,
    }
    
    let clemens = Person{
        name: "clemens".to_string(), 
        age: 26,
    };
    println!("Mein alter ist {0} und mein Name ist {1}", clemens.age, clemens.name);
}