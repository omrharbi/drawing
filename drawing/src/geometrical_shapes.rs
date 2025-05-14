use std::f32::{self, consts::PI};

use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}
#[derive(Debug)]
pub struct Circle {
    pub center: Point, //..
    pub radius: i32,   //..
}
#[derive(Debug)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
    fn color() -> Color {
        Color::rgb(rand::random(), rand::random(), rand::random())
    }
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let c = Point::new(a.x, b.y); // a.x + b.x, b.y
        let d = Point::new(b.x, a.y);

        Self {
            b: b.clone(),
            c,
            d,
            a: a.clone(),
        } // 
    }
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random(width: i32, hight: i32) -> Self {
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..hight);
        Self { x: x1, y: y1 }
    }
}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        Self { start: x, end: y }
    }
    pub fn random(width: i32, hight: i32) -> Self {
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..hight);

        let x2 = rand::rng().random_range(0..width);
        let y2 = rand::rng().random_range(0..hight);
        Self::new(
            Point::new(x1 as i32, y1 as i32),
            Point::new(x2 as i32, y2 as i32),
        )
    }
}
impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            c: Point::new(c.x, c.y),
        }
    }
}

impl Circle {
    pub fn new(x: i32, y: i32, r: i32) -> Self {
        Self {
            center: Point::new(x as i32, y as i32),
            radius: r,
        }
    }
    pub fn random(width: i32, hight: i32) -> Self {
        let x1 = rand::rng().random_range(0..width);
        let y1 = rand::rng().random_range(0..hight);
        let r = rand::rng().random_range(0..hight);
        Self::new(x1, y1, r)
    }
}

impl Drawable for Circle {
    fn draw(&mut self, image: &mut Image) {
        let color = Self::color();
        for i in 0..360 {
            let o = 2.0 * i as f32 * PI / 360 as f32;
            let x = ((self.center.x as f32) + (self.radius as f32) * o.cos()).round();
            let y = ((self.center.y as f32) + (self.radius as f32) * o.sin()).round();

            let o1 = 2.0 * (i + 1) as f32 * PI / 360 as f32;
            let x1 = ((self.center.x as f32) + (self.radius as f32) * o1.cos()).round();
            let y1 = ((self.center.y as f32) + (self.radius as f32) * o1.sin()).round();
            display_color(
                image,
                &Point::new(x as i32, y as i32),
                &Point::new(x1 as i32, y1 as i32),
                color.clone(),
            );
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self, image: &mut Image) {
        Line {
            start: Point::new(self.b.x, self.b.y),
            end: Point::new(self.c.x, self.c.y),
        }
        .draw(image);
        Line {
            start: Point::new(self.c.x, self.c.y),
            end: Point::new(self.a.x, self.a.y),
        }
        .draw(image);
        Line {
            start: Point::new(self.d.x, self.d.y),
            end: Point::new(self.a.x, self.a.y),
        }
        .draw(image);
        Line {
            start: Point::new(self.b.x, self.b.y),
            end: Point::new(self.d.x, self.d.y),
        }
        .draw(image);
    }
}

impl Drawable for Triangle {
    fn draw(&mut self, image: &mut Image) {
        Line {
            start: Point::new(self.a.x, self.a.y),
            end: Point::new(self.b.x, self.b.y),
        }
        .draw(image);
        Line {
            start: Point::new(self.b.x, self.b.y),
            end: Point::new(self.c.x, self.c.y),
        }
        .draw(image);
        Line {
            start: Point::new(self.c.x, self.c.y),
            end: Point::new(self.a.x, self.a.y),
        }
        .draw(image);
    }
}

impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        let color = Self::color();
        display_color(image, &self.start, &self.end, color.clone());
    }
}
fn display_color(image: &mut Image, start: &Point, end: &Point, color: Color) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let step;
    if dx.abs() > dy.abs() {
        step = dx.abs();
    } else {
        step = dy.abs();
    }
    let x_incr = (dx as f32) / (step as f32);
    let y_incr = (dy as f32) / (step as f32);
    let mut x = start.x as f32;
    let mut y = start.y as f32;
    for _ in 0..step {
        Displayable::display(image, x as i32, y as i32, color.clone());
        x += x_incr;
        y += y_incr;
    }
}
impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        let color = Self::color();
        Displayable::display(image, self.x, self.y, color.clone());
    }
}
