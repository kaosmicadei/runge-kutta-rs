use num::Float;

pub struct RK4<T> {
    pub x: T,
    pub y: T,
    pub step: T,
    pub f: fn(T,T) -> T
}

impl<T> Iterator for RK4<T>
    where T: Float
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c2: T = num::cast(2).unwrap(); 
        let c6: T = num::cast(6).unwrap();

        let k1 = self.step * (self.f)(self.x, self.y);
        let k2 = self.step * (self.f)(self.x + self.step/c2, self.y + k1/c2);
        let k3 = self.step * (self.f)(self.x + self.step/c2, self.y + k2/c2);
        let k4 = self.step * (self.f)(self.x + self.step, self.y + k3);
        self.y = self.y + (k1 + c2*k2 + c2*k3 + k4)/c6;
        self.x = self.x + self.step;
        
        Some(self.y)
    }
}

