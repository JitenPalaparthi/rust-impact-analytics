use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    println!("Hello, world!");
    let (mut sx1, mut rx1) = mpsc::sync_channel(10);

    let  sx2 = sx1.clone();
    
    let handle1= thread::spawn(move ||{
        for i in 1..1000{
           // println!("thread 1 sender sending value-->{}",i);
           // thread::sleep(Duration::from_millis(100));
            sx1.send(i);
        }

    });

    let handle2= thread::spawn(move ||{
        for i in 1000..2000{
           // thread::sleep(Duration::from_millis(100));
            sx2.send(i);
        }

    });
    

    let handle3= thread::spawn(move ||{
        //    for v in rx1{
        //  //  thread::sleep(Duration::from_millis(100));
        //    // sx.send(i);
        //   // let v=rx1.recv().unwrap();
        //    println!("Receiver 1 --> Val:={}",v);
        //    }

        loop {
            match rx1.recv() {
                Ok(msg) => println!("Got: {}", msg),
                Err(_) => {
                    println!("Channel closed!");
                    break;
                }
            }
        }
        
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
