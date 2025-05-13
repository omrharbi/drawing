use raster::{Color, Image};

pub trait Displayable {
    fn display() {}
}
pub struct Point {
  pub  x: i32,
  pub  y: i32,
}
impl Point {
   pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(width: i32, hight: i32) -> Self {
        Self {
             x:width,
             y:hight
        }
    }
}
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }
    pub fn random(width: i32, hight: i32) -> Self {


        Self {
            start: Point {
                x: (width),
                y: (width),
            },
            end: Point {
                x: (width),
                y: (width),
            },
        }
    }
}

// pub struct Triangle {
//     pub start: Point,
//     pub mid: Point,
//     pub end: Point,
// }

pub trait Drawable {
    fn draw(&self,image :&Image);
}
impl Drawable for Line {
    fn draw(&self,image :&Image) {
        

    }
}

impl Drawable for Point {
    fn draw(&self,image :&Image) {

    }
}