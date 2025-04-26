fn main() {

let r1=Calculator32::sum(12, 12);
let r2= Calculator64::sum(12,13);
}


struct Calculator32;
struct Calculator64;

trait TCalc<T>{
    type Ouput;
    fn sum(i:T,j:T)->Self::Ouput;
}

impl TCalc<i32> for Calculator32{
    type Ouput = i64;
    fn sum(i:i32,j:i32)->Self::Ouput {
        return (i+j) as i64
    }
}

impl TCalc<i64> for Calculator64{
    type Ouput = i128;
    fn sum(i:i64,j:i64)->Self::Ouput {
        return (i+j) as i128
    }
}

