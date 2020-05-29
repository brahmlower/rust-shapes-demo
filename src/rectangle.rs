#[derive(Debug)]
pub struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    pub fn new(l: i32, w: i32) -> Rectangle {
        Rectangle {
            length: l,
            width: w,
        }
    }
}
