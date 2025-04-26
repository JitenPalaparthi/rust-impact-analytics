pub mod impl_rect;

pub struct Rect{
    pub l:f32,
    pub b:f32
}

 impl Rect {
     pub fn area(&self)->f32{
        return self.l * self.b
    }

    pub fn perimeter(&self)->f32{
        return 2.0*(self.l + self.b)
    }
}