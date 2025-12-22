

fn main() {
    let outer: i32 = 23;

    let cl1 = |i: i32| { i + outer };
    
    let closure_inferred = |i| outer + i;

    let one = || 1;
    println!("{}", one());
    println!("{}", cl1(1));
    println!("{}", closure_inferred(2));
}
