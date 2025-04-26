fn main() {
    println!("Hello, world!");
    let p1 = Person{id:101,name:"Jiten",email:"jitenp@outlook.com",sm:vec!{"linkedin".to_string()}};
    let n = get_name(&p1);
    println!("{}",n);
    fail_scenario();
}

struct Person<'a>{
    id:i32,
    name:&'a str,
    email:&'a str,
    sm:Vec<String>,
}


fn get_name<'a>(p:&'a Person)->&'a str{
    return p.name;
}

fn fail_scenario<'a>(){
    let mut _x = 10;
    let _y = &mut _x; // borrowd value does not live long enough
  //  println!("{}",_x);
        *_y = *_y+1;
        _x = _x+1;
      //  *_y = *_y+1;
}