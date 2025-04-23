use std::{collections::HashMap, hash::Hash};
//use std::collections::hash_map;

fn main() {

 let mut map1 = HashMap::new();

 //let mut map2=HashMap::with_capacity(100);

 map1.insert("560086", "Bangalore-1");
 map1.insert("560096", "Bangalore-2");
 map1.insert("560034", "Bangalore-3");

 for (k,v) in &map1{
    println!("Key:{} Value:{v}",k);
 }

 let val= map1.get("560086324");

//  let v1 = match val{
//     Some(v)=>{
//         v
//     }
//     None =>{
//         ""
//     }
//  };

if let Some(v)=val{
    println!("{}",v);
}else{
    println!("none");
}
  
println!("{}",map1.capacity());
}
