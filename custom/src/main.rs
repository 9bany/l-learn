
#[allow(dead_code)]

#[derive(Debug)]
struct Person{
    name: String,
    age: i32
}

impl Person {
    pub fn print(self) {
        println!("name: {} and age: {}", self.name, self.age);
    }
}

struct Unit;

struct Pair(i32, String);

fn main() {
    let p = Person{ name: String::from("bany"), age: 24};
    p.print();
}
