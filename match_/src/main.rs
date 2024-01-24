
fn match_tuple() {
    let tp = (0, 2, 3);
    match tp {
        (0, x, y) => println!("x: {}, y: {}", x, y),
        _ => println!("default")
    }
}

fn match_arr() {
    let tp = [0, 2, 3];
    match tp {
        [0, x, y] => println!("x: {}, y: {}", x, y),
        _ => println!("default")
    }
}

fn match_struct() {
    struct Food {
        x: (u32, u32),
        y: u32,
    }

    let food = Food{x: (2,3), y: 3};
    match food {
        Food {x: (2, b),..} => {
            println!("b: {}", b)
        },
        _ => println!("default")
    }

    let Food{x: x0, y: y0} = food;
    println!("{:?} {}", x0, y0);
}

fn main() {
    match_tuple();
    match_arr();
    match_struct();
}
