mod mn;
use mn::point as p;

fn main() {
    let mut p  = p::Point::new(0.1, 0.3);
    p.print_val();
    p.increase_x(0.3);
    p.print_val();
}
