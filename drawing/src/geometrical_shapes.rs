mod geometrical_shapes {
    pub trait Drawable {
        fn draw(&self) {}
        fn color() {}
    }
    pub trait Displayable {
        fn display() {}
    }
    pub struct Point {
        pub x: Float,
        pub y: Float,
    }
    impl Point {
        fn new(x: i32, y: i32) -> Self {
            Point { x, y }
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
    }

    pub struct Triangle {
        pub start: Point,
        pub mid: Point,
        pub end: Point,
    }
}
