use std::{thread, time::Duration};
use std::sync::{Mutex,Arc};

// struct Counter(i32);
// struct Rect{
//     l:f32,
//     b:f32
// }
fn main() {
    let fn1 = greet();
    //let c: Mutex<Rect> = Mutex::new(Rect{l:100.12,b:12.4});

    let counter = Arc::new(Mutex::new(0));

    let counter_inc = Arc::clone(&counter);
    let counter_dec = Arc::clone(&counter);
    

let builder= thread::Builder::new().name("thread1".to_string()).stack_size(32 * 1024 * 1024);
    let handle1 = builder.spawn(move || {
        for i in 1..=10 {    
            thread::sleep(Duration::from_millis(500));
            let mut result = counter_inc.lock();
            match result{
                Ok(mut d)=>{
                        *d+=1;
                }
                Err(_)=>{
                    println!("some thing went wrong");
                }
            }
        }
       100
    }).unwrap();

    let handle2 = thread::spawn(move || {
        for i in 1..=10 {
            thread::sleep(Duration::from_millis(550));
          let mut num= counter_dec.lock().unwrap();
          *num -=1;
        }
    });
     
    let val=handle1.join().unwrap();
    handle2.join().unwrap();
    println!("end of main-->{}",*counter.lock().unwrap());
}


fn greet(){
    println!("Hello Wrold");
}