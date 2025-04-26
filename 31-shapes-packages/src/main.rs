use shape::greet;

mod shape;
fn main() {
    // greet();
    use crate::shape::rect::greet::greet;
    crate::greet();
    greet();
}

fn Greet3(){
    println!("Hello world");
}
