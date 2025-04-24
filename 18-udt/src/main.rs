fn main() {

   let p1: Person=Person { id: 101, name: "Jiten".to_string(), email: "JitenP@Outlook.com".to_string() };

   let p2: Box<Person> = Box::new(Person { id: 101, name: "Jiten".to_string(), email: "JitenP@Outlook.com".to_string() });
  
   println!("p2 address:        {:p}",*&p2);
   println!("p2 person  address:{:p}",&p2);

   let a = 10;

    println!("{:p}",&a);

//    let p3: Person = p1;

//    let p4: Box<Person> = p2;

//    println!("p4 address:        {:p}",*&p4);
//    println!("p4 person  address:{:p}",&p4);



}


struct Person{
    id:i32,
    name:String,
    email:String,
}


