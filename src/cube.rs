use crate::traits::{Volume, Mass};

#[derive(Debug)]
pub struct Cube {
    height: i32,
    width: i32,
    length: i32,
    mass: i32
}

impl Cube {
    pub fn new(h: i32, w: i32, l: i32, m: i32) -> Cube {
        Cube {
            height: h,
            width: w,
            length: l,
            mass: m
        }
    }
}

impl Volume for Cube {
    fn volume(&self) -> i32 {
        self.height * self.width * self.length
    }
}

impl Mass for Cube {
    fn mass(&self) -> i32 {
        self.mass
    }
}
