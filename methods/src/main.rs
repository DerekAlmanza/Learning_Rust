fn main() {
    let rect_1 = RectangleDimensions { width: 30, height: 50};
    let rect_2 = RectangleDimensions { width: 10, height: 40 };
    let rect_3 = RectangleDimensions { width: 60, height: 45 };

    // It's clear that `rect_1.area()` is equal to `(&rect_1).area()`
    // or rect_1.can_hold(&rect_2) is equal to (&rect_1).can_hold(&rect_2)
    println!("Can rect1 hold rect2? {}", rect_1.can_hold(&rect_2));
    println!("Can rect1 hold rect3? {}", rect_1.can_hold(&rect_3));
    println!("The rectangle area of the rectangle 1 is: {}", rect_1.area());
}

//Methods are the same of the functions, in fact the implementation of this are equal, with `fn`.
// The difference between both in that they're defined into the context of the struct.
// We redefine our old program of the last lesson. We calculate the rectangle area with methods and
// we check if a rectangle can hold in other rectangle.
#[derive(Debug)]
struct RectangleDimensions {
    width: u32,
    height: u32,
}

// This is a method
// In a method we use `self` syntax, and the program is self-referencing thanks to this. The lenguage
// detects all. If you put `&self` the methos is reading, if you put `&mut self` is mutating, and you
// put `self` is consuming.
impl RectangleDimensions {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rectangle: &RectangleDimensions) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}