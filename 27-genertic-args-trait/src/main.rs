use std::ops::Add;
fn main() {


   use std::string::String;

    println!("Hello, world!");

   let a1= add::<i32>(123,123);
   let a2= add::<i64>(123,123);

   //let a3= add::<bool>(true,false);

   let r1 = Rect::new(12.12,13.12);//i32
   let r2 = Rect::new(12.56,13.45); //f32
   let r3 = add::<Rect>(r1,r2); // rect



   let s1 = add::<MyString>(MyString("hello".to_string()),MyString("World".to_string()));
    println!("{:?}",s1);
}

#[derive(Debug)]
struct MyString(String);


impl Add for MyString{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        MyString(self.0+&other.0)
    }
}

fn add<T: std::ops::Add<Output = T>>(a:T,b:T)->T{
    return a+b;
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

impl Add for Rect {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            l: self.l + other.l,
           b: self.b + other.b,
        }
    }
}
