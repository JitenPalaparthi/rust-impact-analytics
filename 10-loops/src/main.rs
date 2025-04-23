fn main() {
    let mut count: i32 = 0;

    loop {
        count += 1;
        if count > 10 {
            break;
        }
        if count % 2 == 0 {
            continue;
        }
        println!("{}", count);
    }

    let mut count: i32 = 0;

    loop {
        count += 1;
        if count > 10 {
            break;
        }
        if count % 2 == 0 {
            println!("{}", count);
        }
    }

    let mut count: i32 = 0;
    while count <= 10 {
        if count % 2 == 0 {
            println!("{}", count);
        }
        count += 1;
    }
    println!("Nexted loops");

    // let mut done =false;

    //     for i in 1..=5{
    //         if done{
    //             break;
    //         }
    //         for j in 1..=5{
    //             if i+j>8{
    //                 done= true;
    //                 break
    //             }
    //             println!("i:={i} j:{j}");
    //         }
    //     }

    'outer: for i in 1..=5 {
        for j in 1..=5 {
            for k in 1..5 {
                if i + j > 8 {
                    break 'outer;
                }
                println!("i:={i} j:{j} k:{k}");
            }
        }
    }

    let mut count = 0;
    let result: i32 = loop {
        count += 1;
        if count == 5 {
            break count * 2;
        }
    };

}
