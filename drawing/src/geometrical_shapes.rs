use rand::Rng;
use raster::{Color, Image};
pub trait Displayable {
    fn display() {}
}
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    //    pub fn new(x: i32, y: i32) -> Self {
    //         Point { x, y }
    //     }
    pub fn random(width: i32, hight: i32) -> Self {
        Self { x: width, y: hight }
    }
}
#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
impl Line {
    // fn new(start: Point, end: Point) -> Self {
    //     Line { start, end }
    // }
    pub fn random(width: i32, hight: i32) -> Self {
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..hight);

        let x2 = rand::rng().random_range(0..width);
        let y2 = rand::rng().random_range(0..hight);
        Self {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        }
    }
}

// pub struct Triangle {
//     pub start: Point,
//     pub mid: Point,
//     pub end: Point,
// }

pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
}
impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        println!("{:?}", &self);
        while self.start.x <= self.end.x {
            let _ = image.set_pixel(self.start.x, self.end.y, Color::white());
            self.start.x += 1;
        }
    }
}

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        let rands = rand::rng().random_range(1..=4);
        let _ = image.set_pixel(0, 0, Color::rgba(255, 0, 0, 255));
    }
}
