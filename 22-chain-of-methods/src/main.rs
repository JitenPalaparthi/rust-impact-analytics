fn main() {

    let mut c1: Calc= Calc::new(100);

    let d=  c1.add(100).sub(50).mul(2).sub(50).div(2).get();// fluient api or chain of methods
    println!("d:{d}");
    let d=  Calc::new(10).add(20).add(10).get();
    println!("d:{d}");
}

trait TCalc{
    fn add(&mut self,n:i64)-> &mut dyn TCalc;
    fn sub(&mut self,n:i64)-> &mut dyn TCalc;
    fn mul(&mut self,n:i64)-> &mut dyn TCalc;
    fn div(&mut self,n:i64)-> &mut dyn TCalc;
    fn get(&self)-> i64;
}

struct Calc{
    data:i64,
}

impl Calc{
    fn new(d:i64)->Self{
        Self { data: d }
    }
}
impl TCalc for Calc{
    fn add(&mut self,n:i64)-> &mut dyn TCalc {
        self.data+=n;
        return self;
    }
    fn sub(&mut self,n:i64)-> &mut dyn TCalc {
        self.data-=n;
        return self;
    }
    fn mul(&mut self,n:i64)-> &mut dyn TCalc {
        self.data*=n;
        return self;
    }
    fn div(&mut self,n:i64)-> &mut dyn TCalc {
        self.data/=n;
        return self;
    }
    fn get(&self)-> i64 {
        self.data
    }
}