fn main() {

    let mut v1 = vec![1,2];
    let mut v2 = vec![3,4];
    {
    let mut v3 = vec![&mut v1[..],&mut v2[..]];

    // for e1 in &mut v3{
    //     e1.push(100);
    // }

    for e1 in &mut v3{
        e1[0]= 100;
    }
    }

    println!("{:?}",&v1);

    //v3[0][0]= 100;
    

   // println!("{:?}",v3);

}
