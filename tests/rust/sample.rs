struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn of(x: f64, y: f64) -> Self {
        Point::new(x, y)
    }
}

struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn of(center: Point, radius: f64) -> Self {
        Circle::new(center, radius)
    }
}
