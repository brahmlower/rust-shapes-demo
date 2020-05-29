
pub trait Volume {
    fn volume(&self) -> i32;
}

pub trait Mass {
    fn mass(&self) -> i32;
}

pub trait Density {
    fn density(&self) -> i32;
}

impl<T> Density for T where T: Volume + Mass {
    fn density(&self) -> i32 {
        self.mass() / self.volume()
    }
}