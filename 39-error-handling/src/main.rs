use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file_resule = File::open("hello.txt");
    match file_resule{
        Ok(mut file)=>{
         let mut contents = String::new();
         file.read_to_string(&mut contents)?;
         Ok(contents) 
     }
     Err(e)=>
        Err(e)
     }

}

fn main() {
    match read_file() {
        Ok(data) => println!("File contents:\n{}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}