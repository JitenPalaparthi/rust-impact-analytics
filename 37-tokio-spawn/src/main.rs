use std::time::Duration;


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main(){

    
    let join3=tokio::task::spawn_blocking(|| print_numbers1("separate thread-1"));
    let join1= tokio::spawn(print_numbers1("str1"));
    let join2 = tokio::spawn(async {
        for i in 1..100{
            println!("Task--2 {}",i);
        }
    });

    join1.await.unwrap();
    join2.await.unwrap();
    join3.await.unwrap();
}


async fn print_numbers1(str1:&str){
    for i in 1..100{
       // tokio::time::sleep(Duration::from_millis(100));
       let task = async ||->i32{
        let s = i * i + ( i +2)/10+100;
        return s;
       };
      let r= task().await;
      println!("task-- {} {} {} ",str1,i,r);
    }
}
