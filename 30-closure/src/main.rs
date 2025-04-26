fn main() {
    println!("Hello, world!");

    let r1 = Calc(10, 20, |a1: &i32, b1: &i32| -> i32 {
        return *a1 + *b1;
    });

    println!("{}", r1);

    let r2 = Calc(10, 20, sub);
    println!("{}", r2);

    let fn2 = |a1: &i32, b1: &i32| -> i32 {
        return *a1 * *b1;
    };

    let r3 = Calc(10, 20, fn2);
    println!("{}", r3);

    let mut vec1 : Vec<Box<dyn Fn()>> = Vec::new();
    vec1.push(Box::new(|| {
        println!("Hello World");
    }));

    let fn1 = get_sq(100);
    let fn2 = fn1();

    let r1 = fn2();

    println!("Result:{}",r1);

  //  let s2 = fn1();
}

fn Calc(a: i32, b: i32, fn1: impl Fn(&i32, &i32) -> i32) -> i32 {
    fn1(&a, &b)
}

fn sub(i: &i32, j: &i32) -> i32 {
    return *i - *j;
}

fn get_sq(n: i32) -> Box<dyn Fn() -> Box<dyn Fn() -> i32>> {
    Box::new(move || {
        Box::new(move || n * n)
    })
}

fn get_sq1(n: i32) -> impl Fn() -> Box<dyn Fn() -> i32> {
    move || {
        Box::new(move || n * n)
    }
}