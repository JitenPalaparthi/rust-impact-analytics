fn main() {
    let arr1: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    //let sum = Sumof(&arr1);

    let slice1 = &arr1[..];
    let slice2 = &arr1[1..];
    let slice3 = &arr1[1..5];
    let slice4 = &arr1[1..=5];

    let vec1 = vec![10, 20, 40, 50, 60];
    let slice5 = &vec1[..];

    let sum = Sumof(&arr1);
    println!("sum:{}",sum);

    let sum = Sumof(&vec1);
    println!("sum:{}",sum);

    let sum = Sumof(&slice1);
    println!("sum:{}",sum);


    let str1: &str="Hello world";
    let str2: String = "hello World".to_string();
    let str3 = &str2;
     
    let l=get_len_of(str3);

    println!("length: {}",l);

    let l=get_len_of(str1);

    println!("length: {}",l);
}

fn SumofArr(arr: &[i32; 5]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    sum
}


fn Sumof(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for v in arr {
        sum += v;
    }
    sum
}

fn get_len_of(s: &str)->usize{
    s.len()
}

fn get_len_of1(s: &String)->usize{
    s.len()
}

