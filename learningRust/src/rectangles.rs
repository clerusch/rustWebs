trait Area {
    fn area(&self) -> u32;
}
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new(width:u32, height:u32)->Self{
        Self {width, height}
    }
}
impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
}