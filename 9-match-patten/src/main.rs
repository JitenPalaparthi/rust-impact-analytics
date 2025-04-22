fn main() {
    let day: u8 = 100;

    match day {
        1 => {
            println!("Sunday");
        }
        2 => {
            println!("Monday");
        }
        3 => {
            println!("Tuesday");
        }
        4 => {
            println!("Wednesday");
        }
        5 => {
            println!("Thursday");
        }
        6 => {
            println!("Friday");
        }
        7 => {
            println!("Saturday");
        }
        _ => {
            println!("No day");
        }
    }

    // match patten with conditions

    let num = 100;

    match num {
        n1 if n1 % 2 == 0 => println!("{n1} is even number"),
        n2 if n2 % 2 != 0 => println!("{n2} is odd number"),
        _ => {}
    }


    let char1:char = 'A';

    match char1{
        'a' | 'e' | 'i' |'o' | 'u' => println!("{char1} is lower case vovel"),
        'A' | 'E' | 'I' |'O' | 'U' => println!("{char1} is upper case vovel"),
        _ => println!("{char1} is consonent or a special char")
    }


    let num:i32 = 10;

    match num{
       1 | 2 | 3 | 4 | 5 if num%2==0 => {
        println!("{num} is even")
       },
       6 | 7 | 8 | 9 | 10 if num%2!=0 => {
        println!("{num} is odd ")
       },

        _ => println!("{num} is beyond 10")
    }



}
