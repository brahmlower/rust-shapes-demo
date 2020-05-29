use crate::traits::{Volume, Mass};

#[derive(Debug)]
pub struct Sphere {
    radius: i32,
    mass : i32
}

impl Sphere {
    pub fn new(r: i32, m: i32) -> Sphere {
        Sphere {
            radius: r,
            mass: m
        }
    }
}

impl Volume for Sphere {
    fn volume(&self) -> i32 {
        let pi = 3;
        3 * pi * (self.radius * self.radius)
    }
}

impl Mass for Sphere {
    fn mass(&self) -> i32 {
        self.mass
    }
}
