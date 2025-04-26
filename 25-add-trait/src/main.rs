use std::ops::Add;
fn main() {

    let r1 = Rect::new(10.12, 12.34);
    let r2 = Rect::new(10.12, 12.34);

    //let r3 = r1+r2;
    let r3= r1.add(r2);

    //let r2=r1.Add(r2);
}



#[derive(Debug)]
struct Rect{
    l:f32,
    b:f32,
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        return Self { l: l, b: b };
    }
}


pub trait Add1<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}


impl Add for Rect {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            l: self.l + other.l,
           b: self.b + other.b,
        }
    }
}


