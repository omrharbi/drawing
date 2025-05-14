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
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let c = Point::new(a.x, b.y); // a.x + b.x, b.y
        let d = Point::new(b.x, a.y);
        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("c: {:?}", c);
        println!("d: {:?}", d);
        println!("-----------------");
        Self {
            b: b.clone(),
            c,
            d,
            a: a.clone(),
        } // 
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

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            c: Point::new(c.x, c.y),
        }
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
        let step;
        if dx.abs() > dy.abs() {
            step = dx.abs();
        } else {
            step = dy.abs();
        }
        let x_incr = (dx as f32) / (step as f32);
        let y_incr = (dy as f32) / (step as f32);
        let mut x = self.start.x as f32;
        let mut y = self.start.y as f32;
        for _ in 0..step {
            Displayable::display(image, x as i32, y as i32, Color::white());
            x += x_incr;
            y += y_incr;
        }
    }
}

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        let color: Color = Color::green();
        Displayable::display(image, self.x, self.y, color);
    }
}
