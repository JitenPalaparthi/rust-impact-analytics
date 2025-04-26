use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    let mut count = 100;
    println!("{:p}",&count);
    let handle1 = thread::spawn(move || {
        for i in 1..10{
        count += 1;
        //println!("{:p}",count);
        thread::sleep(Duration::from_millis(1000));
        println!("therad-1-->{:p} --> {}",&count,count);
        }

    });

    let handle2 = thread::spawn(move || {
        for i in 1..10{
            count += 1;
            thread::sleep(Duration::from_millis(1000));
            println!("therad-2-->{:p} --> {}",&count,count)
            }
    });

   
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}",count);
}
