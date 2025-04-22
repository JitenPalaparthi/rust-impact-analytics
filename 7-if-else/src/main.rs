fn main() {
    let age: u8 = 18;

    if age >= 18 {
        println!("eligible for vote");
    } else {
        println!("not eligible for vote");
    }

    // if elseif else
    let age: u8 = 18;
    let gender: char = 'F';

    if age >= 18 && (gender == 'F' || gender == 'f') {
        println!("she is eligible for marriage");
    } else if age >= 21 && (gender == 'M' || gender == 'm') {
        println!("she is eligible for marriage");
    } else {
        println!("not eligible");
    }

    let marriage_status = if age >= 18 && (gender == 'F' || gender == 'f') {
        "she is eligible for marriage"
    } else if age >= 21 && (gender == 'M' || gender == 'm') {
        "she is eligible for marriage"
    } else {
        "not eligible"
    };

    let char1: Option<char> = Some('A');

    if char1.is_some() {
        let char2 = char1.unwrap();
        println!("{}", char2);
    }

    //_ = 42;// blank identifer
    // true
    // "Hello World"

    let mut some_value = Some(5);

    match some_value {
        Some(v) => println!("{}", v),
        None => println!("no value"),
    }
    some_value = None;

    
    let some_sq_value = if let Some(v) = some_value { 
        v * v 
    } else {
         0 
        };

        println!("{}",some_sq_value);


        //println!("{:?}",Some(100));

}
