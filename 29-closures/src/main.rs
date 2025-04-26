fn main() {
    println!("Hello, world!");

    let greet = || {
        println!("Hello world");
    };
    greet();

    (|| {
        for i in 1..=10 {
            print!("{i} ");
        }
    })();

    let func1 = |i: i32, j: i32| -> i32 { i + j };

    let s1 = func1(10, 20);

    let s2 = (|i: i32, j: i32| -> i32 { i + j })(12, 13);

    let mut count = 0;

    let mut fnmut1 = || {
        count += 1;
    };

    fnmut1();
    println!("Count:{}",count);

    let mut vec1= vec![10,20,30,40];
    let str1 = "Hello WORld".to_string();

    let mi1 = MyInt(100);

    let mi2=mi1.clone();

    let mut fnmut1 = move || {
       vec1.push(count);
       for i in vec1{
        println!("{}",i);
        println!("{:?}",mi1);
       // println!("{}",str1);
       }
    };

   let r= fnmut1();
   println!("Count:{}",count);
   println!("{}",str1);
   println!("{:?}",mi1);

   let mi3 = &mi1;

   let mi4 = *mi3;

  let mi5= mi3.clone();


}

// Fn
// FnMut
// FnOnce 

#[derive(Debug)]
struct MyInt(i32);

impl Copy for MyInt{}

impl Clone for MyInt {
    fn clone(&self) -> MyInt{
        *self
    }
}

struct Rect{
    l:f32,
    b:f32,
}

impl Copy for Rect{}

impl Clone for Rect {
    fn clone(&self) -> Rect{
        *self
    }
}