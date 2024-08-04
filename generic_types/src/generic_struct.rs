#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    #[allow(dead_code)]
    fn x(self: &Self) -> &T {
        &self.x
    }

    #[allow(dead_code)]
    fn y(self: &Self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
pub struct Point2<T, U> {
    pub x: T,
    pub y: U,
}

impl<T,U> Point2<T,U> {
    pub fn mixup<V,W>(self, other: Point2<V,W>) -> Point2<T,W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
