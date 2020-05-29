mod cube;
mod sphere;
mod rectangle;
mod traits;

use cube::Cube;
use sphere::Sphere;
use rectangle::Rectangle;
use traits::{Volume, Density};

fn printer<T>(item: T)
where
    T: std::fmt::Debug + Volume + Density
{
    println!("Item is: {:?}", item);
    println!("Item volume is: {:?}", item.volume());
    println!("Item density is: {:?}", item.density());
}

fn main() {
    let c = Cube::new(1, 2, 3, 20);
    let s = Sphere::new(2, 20);
    let r = Rectangle::new(3, 4);
    printer(c);
    printer(s);
    printer(r);
}
