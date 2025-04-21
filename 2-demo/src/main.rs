
static mut count:i32 = 100; // global variables

#[allow(static_mut_refs)]
fn main() {
    println!("Hello, world!");

    let mut str1 = "Hello Wrold"; // the actual data is sstored in the data segment and the string variable is created in the stack mmemory
   
    let str2 = "你好，世界";
    {
        let str3: String= String::from("Hello World"); // another string which is heap allocated
    }

    // drop str3 
    // borrow  checker

    str1 = "hello Universe!";

    println!("{} {}",str1,str2);

    unsafe {
     count+=1;
     println!("count:{}",count);
    }
}
// &str

