fn main() {
    let mut vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];
    // stack heap

    let vec2 = Vec::<i32>::new();

    let vec3 = Vec::<i32>::from(arr1);

    for v in arr1 {
        println!("{}", v);
    }
    arr1[0] = 999;

    for v in &vec1 {
        // 1 immutable borrow
        println!("{}", v);
    }
    vec1.push(100); // so there cannot be mutable 
    // there can be either 1 mutable borrow or any number of immutable borrows
    vec1.pop();
    vec1.remove(2);

    for v in &vec1 {
        println!("{}", v);
    }

    let a = {
        let (a, b) = (100, 200);
        a + b / a - b * (a + 2) * (b / 2)
    };

    println!("{:?}", a);

    let a = {
         0
         };

        //  for n in vec1.iter_mut().enumerate(){ 
        //  }
    

       // let e1= vec1[10];
        let e2= vec1.get(3);

       let val= if let Some(v)=e2{
            println!("{}",v);
           // *v = *v * *v;
            *v
        }else{
            //println!("no element at the index")
            0
        };


        // let mut a = 100;

        // let ao :Option<&mut i32> = Some(&mut a);

}
// allocator
// general // areana allocators, heap allocator
// Zig
// Vault -- disable swap

fn add_and_sum_of(v: &mut Vec<i32>, elem: i32) -> i32 {
    let mut sum = 0;
    v.push(elem);
    for i in v {
        sum += *i;
    }
    sum
}

// ; considerd as a statement
// no ; considered as an expression