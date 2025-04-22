fn main() {
    let str1 = "Hello";
    let str2 = " World!";

    let str3 = (str1.to_string() + str2).to_string() + "Hello";

    println!("Hello World");

    let str6 = "Hello Woirld";


    let s1= "Hello".to_string();
    // let s2 = s1; // move when the move happens the ownership is transffered

    let (l,s1)=get_length(s1);
    println!("{}",s1);
        let s2: &String = &s1; // borrow

        println!("{}",s1);   
        println!("{}",s2);  

        let l = get_lengthR(&s2);
        println!("{}",s1);   

}


fn get_length(s:String)->(usize,String){
        return (s.len(),s);
}


fn get_lengthR(s:&String)->usize{
    //return (&s).len();
    s.len()
}
