fn main() {
    let num1: i8 = 124;

    let num2: i64 = num1 as i64; // explicitely type cast 

    let char1: char = 'A';

    let ok1 = true;

    let num3: u8 = char1 as u8;

    println!("{}", num3);

    let char2 = '😊';

    let num4: u8 = char2 as u8;

    println!("{}", num4);

    let num5: u32 = 35791120; //0b10001000100010000100010000; // 35791120
    println!("{}", num5);
    let num6: u8 = num5 as u8; // 0b0010000
    println!("{}", num6);

    // char::from_u32()

    let c = '💯';
    let i = c as u32;

    println!("{}", i);

    let num7 = 67 as u8;

    let num8: i32 = 19000;

    let char3: char = num7 as char;

    let char4 = char::from_u32(num8 as u32).unwrap();

    println!("char5:{}", char4);

    // Some
    // None
    // Option

    let char5: Option<char> = Some('a');
    let char6: Option<char> = None;

    //let char7 = char6.unwrap();

    match char6 {
        Some(v) => println!("{}", v),
        None => {
            println!("None no value ")
        }
    }

    //*int ptr;

    //    let a = &100;
    //    let b = &a;

    //. let c: &i32 ;
    // println!("{}",c);

    //   let mut a :Option<i32> = Some(0);
    //   a = None;
}
