fn main() {
    println!("Hello, world!");

    let d = Direction::Bottom;

    match d {
        Direction::Left => println!("Direction d is left {}", d as u8),
        Direction::Right => println!("Direction d is right {}", d as u8),
        Direction::Top => println!("Direction d is top {}", d as u8),
        Direction::Bottom => println!("Direction d is bottom {}", d as u8),
    }

    let m1 = Message::Move { x: 100, y: 200 };
    print_message_enum(m1);
    let m2 = Message::Write("Hello World".to_string());
    print_message_enum(m2);

    let m3 = Message::Quit;
    print_message_enum(m3);

    let mut m4 = Message::RGB(100, 120, 240);
    print_message_enum(m4);


    m4 = Message::Write("Hello World".to_string());
    print_message_enum(m4);


    // match m4 {
    //     Message::Quit => {
    //         println!("Quit is called");
    //     }
    //     Message::Move { x, y } => {
    //         println!("x:{x} y:{y}");
    //     }
    //     Message::RGB(a, b, c) => {
    //         println!("a:{a} b:{b} c:{c}");
    //     }
    //     Message::Write(s) => {
    //         println!("S:{s}");
    //     }
    // }
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

enum Message {
    Quit,
   // QuitO,
    Move { x: i32, y: i32 }, // struct
    MoveM { x: f64, y: f64}, // struct
    Write(String),           // unit struct
    RGB(u8, u8, u8),         // tuple
}

fn print_message_enum(m:Message){
    match m {
        Message::Quit => {
            println!("Quit is called");
        }
        Message::Move { x, y } => {
            println!("x:{x} y:{y}");
        }
        Message::MoveM { x, y } => {
            println!("x:{x} y:{y}");
        }
        Message::RGB(a, b, c) => {
            println!("a:{a} b:{b} c:{c}");
        }
        Message::Write(s) => {
            println!("S:{s}");
        }
    }
}