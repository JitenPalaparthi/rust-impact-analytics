use std::collections::btree_map::Values;

fn main() {
    let mut arr1: [i32; 5] = [10, 12, 34, 56, 67];

    arr1[4] = 999;

    let mut sum = 0;
    for i in arr1 {
        sum += i;
    }

    println!("Sum of elements :{}", sum);

    let mut arr2: [i32; 5] = [0; 5]; // create an array for 5 elements with each element is 0. array with default values
    // let mut arr3: [i32; 9]=[0;9]; // create an array for 5 elements with each element is 0. array with default values

    println!("{:?}", arr2);

    for (index, val) in arr1.iter_mut().enumerate() {
        println!("index: {} value: {}", index, val);
        *val *= *val;
    }
    println!("{:?}", arr1);

    for (index, val) in arr1.iter_mut().enumerate() {
        println!("index: {} value: {}", index, val);
        *val /= *val;
    }
    println!("{:?}", arr1);

    let arr2d = [[1, 2], [3, 4]];

    for ar in arr2d{
        for e in ar{
            println!("{}",e);
        }
    }

    let arr3d = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    
    for ar1 in arr3d{
        for ar2 in ar1{
            for e in ar2{
                println!("{}",e);
            }
        }
    }
    
}

// self
// this
