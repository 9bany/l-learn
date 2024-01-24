fn create_fn() -> impl Fn() {
    return move || println!("hello world!!")
}

fn main() {
    let print = create_fn();
    print();
}
