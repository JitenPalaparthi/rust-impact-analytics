fn main() {

let s1 = Square(12.34);
println!("area : {}",s1.area());
println!("perimter: {}",s1.perimeter());

let e1 = empty1; // zero size
let v1=e1.sq(100);
empty1::area(100.23,123.4);
empty2::area(12.2,23.4,23.4);

}

//  Tuple struct
 #[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rect(f32,f32);


impl Square{
    fn new(s:f32)->Self{
        Self(s)
    }

    fn area(&self)->f32{
        self.0 *self.0
    }

    fn perimeter(&self)->f32{
        self.0 * 4 as f32
    }
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Self(l,b)
    }

    fn area(&self)->f32{
        self.0 *self.1
    }

    fn perimeter(&self)->f32{
        (self.0+self.1 )*2 as f32
    }
}

struct empty1;

struct empty2;

// signal passing using channels
// to statisfy interfaces
// kind of a polymorhism

impl empty1{
    fn sq(&self,n:i32)->i64{
        n as i64 * n as i64
    }
    fn area(l:f32,b:f32)->f32{
        return l * b
    }
}


impl empty2{
    fn area(l:f32,b:f32,h:f32)->f32{
        return l * b *h
    }
}

struct T1{
    ok:bool, // ok - - - | num | done - - - -
    num:u32, 
    done:bool,
} // 12 bytes

// 4  | 4   | 4

struct T2{
    num:u32, // num | ok done - -|
    ok:bool,
    done:bool,
} // 8 bytes
