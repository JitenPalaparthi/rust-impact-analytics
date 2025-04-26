use std::{thread, time::Duration};
fn main() {

    let fn1 = greet();
    let handle1 = thread::spawn(|| {
        for i in 1..=100 {    
            thread::sleep(Duration::from_millis(500));
            println!("Thread -1 {}", i);
        }

       100
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=100 {
            thread::sleep(Duration::from_millis(550));
            println!("Thread -12 {}", i);
        }
    });

    let result = handle1.join();

    match result {
        Err(_) => {
            println!("there seems to be some issue or error")
        }
        Ok(v) => {
            println!("The ultimate result:{}", v)
        }
    }

     

    handle2.join().unwrap();
    println!("end of main")
}


fn greet(){
    println!("Hello Wrold");
}