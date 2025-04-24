use std::path::Display;

fn main() {

    let r1: Rect= Rect::new(12.34,12.34);
    let s1= Square(12.24);

    print_shape1(r1); // prinnt_shapre_rect
    print_shape2(s1); 

    let r3: Rect= Rect::new(12.34,12.34);

    print_shape3(r3); // prinnt_shapre_rect

    let r2: Rect= Rect::new(12.34,12.34);
    print_shape4(&r2);

    let s2= Square(12.24);
    print_shape4(&s2);

}

trait TShape :TWhat{
    fn area(&self) -> f32{
        return 0.0;
    }

    fn perimter(&self) -> f32;
}

trait TWhat{
    fn what(&self)->String;
}
struct Rect{
    l:f32,
    b:f32,
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Rect { l: l, b: b }
    }
}



impl TShape for Rect{
    fn area(&self) -> f32 {
        return self.l * self.b;
    }
    fn perimter(&self) -> f32 {
        return 2.0 * (self.l+self.b);
    }
}

impl TWhat for Rect{
    fn what(&self)->String {
        return "Rect".to_string();
    }
}



// impl Display for Rect{

// }

struct Square(f32);

impl TShape for Square{
    fn area(&self) -> f32 {
        return self.0 * self.0;
    }
    fn perimter(&self) -> f32 {
        return 4.0* (self.0);
    }
}


impl TWhat for Square{
    fn what(&self)->String {
        return "Square".to_string();
    }
}

fn print_shape1(t : impl TShape){
    println!("Area:{}",t.area());
    println!("Perimeter:{}",t.perimter());
}

fn print_shape2<T: TShape>(t:T){
    println!("Area of {} :{}",t.what(),t.area());
    println!("Perimeter of {} :{}",t.what(),t.perimter());
}

fn print_shape3<T>(t:T) where T:TShape{
    println!("Area of {} :{}",t.what(),t.area());
    println!("Perimeter of {} :{}",t.what(),t.perimter());
}

fn print_shape4(t : &dyn TShape){
    println!("Area:{}",t.area());
    println!("Perimeter:{}",t.perimter());
}

// Vtable
// origincal object, pointer to the method

// fn print_shape_rect(t: Rect){
//     println!("Area:{}",t.area());
//     println!("Perimeter:{}",t.perimter());
// }


// fn print_shape_square(t: Square){
//     println!("Area:{}",t.area());
//     println!("Perimeter:{}",t.perimter());
// }

//9618558500
