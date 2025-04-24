fn main() {

    let mut n = 100;

    let n1 = getSq1(&mut n);

    // let n2 =n ;
    println!("{}",n);

    //let n2 = getSq2(n);

} ///


fn getSq1<'a>(mut n: &'a mut i32)->&'a i32{ // new n is created
    *n = *n * *n ;
    return n; // dangling pointer
}

fn longest<'impact >(s1:&'impact str, s2:&'impact str)->&'impact str{
    if s1.len()>s2.len(){
        s1 
    }else{
        s2
    }
 }

// fn getSq2<'a> (mut n: i32)->&'a i32{ // new n is created
//     n = n * n ;
//     return &n; // dangling pointer
// }

fn getSq2(mut n: i32)->Box::<i32>{ // new n is created
    n = n * n ;
    return Box::new(n); // dangling pointer
}


// func getSquare(n int)*int{
//     n1 := new(int)// creating a pointer
//     *n1 = n * n
//     return n1 // move the variable n1 to heap
// } // escape anlays