const PI: f32 = 3.14; // code

static global: i32 = 12312;

fn main() {

    const PI2:f32 = 3.145; // may be in data segment
    println!("Hello, world!");
    let num1: i64 = -12312321312; // mark to create a varialbe for 8 bytes in stack
    let mut num5: i32 = 0;

    println!("num1:{}", num1);
    {
        // sf2
        let num2: i32 = 12321; // allocated by whom? Where it gets allocated 
        let ok1: bool = true;
        println!("num2:{} ok1:{}", num2, ok1);
        num5 = num2;
    }
    // println!("num2:{} ok1:{}",num2,ok1);
    let mut num6 = 0;
    num6 = num5;

    let mut num7 = 123123;

    num7 = 134439343;

    let (num3, num4) = (1000, 200);
    println!("num3:{} num4:{}", num3, num4);

    let t: (i32, &str, bool, f64) = (41,"Jiten",true,123.1231);
    println!("Age:{} name:{} something:{} someting:{}",t.0,t.1,t.2,t.3);


    let(t1,t2,t3,t4)=t;
    println!("Age:{} name:{} something:{} someting:{}",t1,t2,t3,t4);

    // always it returns 0
}
// 0 non-0
// preprocessing

// Numbers
/*
i8,i16,i32,i64,i28
u8,u16,u32,u64,u128
f32,f64
isize,usize
*/

// boolen --> bool
// character --> char

/* Strings
&str -> data segment allocated string slice
String-> heap allocated string
*/
