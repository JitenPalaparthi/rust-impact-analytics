pub struct Square{
    pub s:f32,
   
}

 impl Square {
     pub fn area(&self)->f32{
        return self.s * self.s;
    }

    pub fn perimeter(&self)->f32{
        return 4.0* self.s;
    }
}