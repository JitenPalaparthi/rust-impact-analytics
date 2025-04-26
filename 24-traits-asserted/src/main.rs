

fn main() {
    {
     let r1 = Rect::new(10.12, 12.34);
        println!("{:?}",r1);
        let r2 = Box::new(r1); // moving r1 to heap memory
        println!("{:?}",r2);
    }
    println!("end of main");
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


impl Drop for Rect{
    fn drop(&mut self){ // drop is not going to drop the object , instead drop is like a 
        println!("Reect is getting dropped");
    }
}
