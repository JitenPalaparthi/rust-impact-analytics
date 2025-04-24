fn main() {
    println!("Hello, world!");

    let f1 = || {
        println!("Hello World");
    };

    f1();

   (|s:&str|{
        println!("{}", s);
    })("Hello World");

    let mut vec1= vec![10,20,30,40,50];

    let fn2=||{
        vec1.push(200);
        let mut sum=0;
        for v in vec1{
            sum+=v;
        }
        println!("sum:{}",sum);
    };

    let v = getref();
    println!("{}",v);


    let b = Box::new(100);

    // Heap address: where the value is stored
    let heap_ptr: *const i32 = &*b;

    // Stack address: where the Box<T> lives (just the pointer on stack)
    let stack_ptr: *const Box<i32> = &b;

    println!("Heap address:  {:p}", heap_ptr);
    println!("Stack address: {:p}", stack_ptr);
    println!("Stack address: {:p}", b);

    let a = 100;
    println!("Stack of a:    {:p}", &a);


    let str1 ="Hello Wrold";

    println!("addr str1:     {:p}",str1);
    println!("addr str1:     {:p}",&str1);


}

fn getRef<'a>(n:&'a i32)->&'a i32{
    return n;
}

// fn getref<'a>()->&'a i32{
//     let n = 100;
//     let k = &n;
//     return k;
// }

fn getref<'a>()-> Box<i32>{
    return Box::new(100);
}