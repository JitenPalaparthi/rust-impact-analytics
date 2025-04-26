use std::ops::{Add, Mul};
fn main() {
    println!("Hello, world!"); 

    let r1: Rect<i32> = Rect::<i32>{l:100,b:100};
    let r1: Rect<f32> = Rect::<f32>{l:100.123,b:100.123};

}


struct Rect<T>{
    l:T,
    b:T
}

impl<T> TArea for Rect<T> where T:Mul<Output =T>+Copy{
    type Output = T;
    fn area(&self)-> T{
        return self.l*self.b;
    }
}

trait TArea{
    type Output;
fn area(&self)-> Self::Output;
}
