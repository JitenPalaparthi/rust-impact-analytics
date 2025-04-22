fn main() {

    let mut s1:String = String::from("Hello World");
    //let mut s2 = "Hello World";

   // {
    let s2 = &s1;
    let s3 = &s1;
    let s4=  &s1;
    println!("{} {} {} {}",s1,s2,s3,s4);
    //}

   // s1.push_str("Hey");
   let s5: &mut String = &mut s1;

   //  println!("{} {} {}",s2,s3,s4);
   s5.push_str(" How are you doing");

   let s6 = s5; 

   s6.push_str(" How are you doing");

   // println!("{}",s5);

}
