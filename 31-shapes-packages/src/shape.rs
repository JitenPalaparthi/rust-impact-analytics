pub fn greet(){
    println!("Hello World");
    crate::shape::rect::greet::greet2();
    super::Greet3();
}

pub mod greeting{
   fn greet(){
        println!("Hello World");
   } 
}

pub mod circle{
    pub struct Circle(f32);
}

pub mod triangle{
    pub struct Triangle(f32,f32);
}

pub mod rect;
pub mod square;