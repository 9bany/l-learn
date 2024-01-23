
use std::mem;
fn apply<F>(f: F) where F: FnOnce() {
    f()
}

fn apply_3<F>(f: F) -> i32 where 
    F: Fn(i32) -> i32  {
    f(3)
}

fn main() {
    let dirany = || {
        let mut greeting = "helloworld".to_owned();
        println!("{}", greeting);
        greeting.push_str("!!!!");
        println!("{}", greeting);
        mem::drop(greeting);
    };

    apply(dirany);
}
