type U64 = u64;
fn main() {
    let fnum = 2.9f32;
    let inum: i32 = fnum as i32;
    let unum = fnum as U64;
    println!("{} {}",inum, unum);
}
