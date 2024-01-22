pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn print_val(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    pub fn increase_x(&mut self, x: f64) {
        self.x += x;
    }

}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        return Point{x: x, y: y}
    }
}