fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    let upper = 1000;
    let result: u32 = (0..)
    .map(|n| n * n)
    .take_while(|&nsuareed| nsuareed < upper).filter(|&n| is_odd(n)).sum();

    println!("result is {}", result);
}
