use std::time::Duration;


#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main(){
    let task1 = print_numbers1("task1"); // join handle 
    let task2 = print_numbers2("task2");

    let task3 = async {
        for i in 1..100{
            tokio::time::sleep(Duration::from_millis(100));
            println!("task-- task3 {}",i);
        }
    };

     async{
        100
    }.await;

    let fn1 = async ||{
        for i in 1..100{
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("task-- task4 {}",i);
        }
    };
    let task4 = fn1();

    tokio::join!(task1,task4,task2,task3);
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

async fn print_numbers2(str1:&str){
    for i in 1..100{
      //  tokio::time::sleep(Duration::from_millis(100));
        println!("task-- {} {}",str1,i);
    }
}