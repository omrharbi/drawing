use rand::Rng;
use raster::{Color, Image};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Point {
    pub fn random(width: i32, hight: i32) -> Self {
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..hight);

        Self { x: x1, y: y1 }
    }
}

impl Line {
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

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
}
impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let mut step = 0;

        if dx.abs() > dy.abs() {
            step = dx.abs();
        } else {
            step = dy.abs();
        }

        let x_incr = dx / step;
        let y_incr = dy / step;

        let mut x = self.start.x;
        let mut y = self.start.y;

        // let color=Color::white();
        for _ in 0..step {
            // console.log(round(x) + " " + round(y));
            Displayable::display(image, x as i32, y as i32, Color::white());
            x += x_incr;
            y += y_incr;
        }
            //  while self.start.x <= self.end.x {
        //     self.start.x += 1;
        // }
    }
}

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        let color = Color::green();
        Displayable::display(image, self.x, self.y, color);
    }
}
