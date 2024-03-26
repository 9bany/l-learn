use std::ops::Add;

enum Math {
    Add,
    Subtract,
}

impl Math {
    pub fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Math::Add => x + y,
            Math::Subtract => x - y,
        }
    }
}
fn main() {
    println!("{}", Math::Add.run(2, 3));
    println!("{}", Math::Subtract.run(2, 3));
}