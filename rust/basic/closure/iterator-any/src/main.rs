use std::vec;

fn main() {
    let vec1 = vec![1, 2, 3];
    println!("Are 2 in vec ? {}",vec1.iter().any(|&x| x == 2));
    println!("Are 3 in vec ? {:?}", vec1.iter().find(|&&x| x == 3));
}
