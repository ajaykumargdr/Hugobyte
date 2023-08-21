use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// single type (point + point)
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 2 different types (MM + M)
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {

    let x = Point{ x:1, y:0 };
    let y = Point{x: 2, y:3 };

    assert_eq!(x+y, Point{ x:3, y:3 });

////////////////////////////////////////

    let mm = Millimeters(1200);
    let m = Meters(1);

    let mm2 = mm + m;

    println!("{:?}", mm2);

}
