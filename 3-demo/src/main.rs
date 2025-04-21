fn main() {
    println!("Hello, world!");

    let a = 100;
    let b = a;
    // copy trait


    let s1 = "Hello World".to_string();
    let s2 = s1;


    println!("a:{} b:{}",a,b);

    println!("{}",s2);

    let mut a: &mut i32 = &mut 1000; // memory is allocated and address is given to a

    let b: &mut i32 = a; // is move or a copy

    *b = 3000;

   //println!("{}",a);
    println!("{}",*b);

    {
        let a = &true;
        println!("a:{:p}",a);
    }


}

// move 
// copy
