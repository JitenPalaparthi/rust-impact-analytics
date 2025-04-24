use std::fmt::{Display,Formatter,Result};
fn main() {

    let mut r1 = Rect::new(100.25,123.45);
    let a1 = r1.area();
    let p2 = r1.perimeter();
    println!("{}",r1);
}

//std::fmt::Display

#[derive(Debug)]
struct Rect{
    l:f32,
    b:f32,
    a:f32,
    p:f32
}

impl Rect{
    fn new(l:f32,b:f32)-> Self{
        Rect { l: l, b: l ,a:0.0,p:0.0}
    }

    fn default()->Rect{
        Rect { l: 1.0, b: 1.0,a:0.0,p:0.0 }
    }
}

impl Rect{
    fn area(&mut self)->f32{ // self --> this
        self.a = self.l * self.b;
        return self.a;
    }

    fn perimeter(&mut self)->f32{ // self --> this
        self.p= 2.0 * (self.l + self.b);
        return self.p;
    }
}
    impl Display for Rect {
         fn fmt(&self, f: &mut Formatter<'_>) -> Result {
             write!(f, "(Length:{}, Width:{} Area: {} Perimeter: {} )", self.l, self.b,self.a,self.p)
         }
     }

