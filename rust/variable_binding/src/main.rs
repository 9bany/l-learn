fn main() {
    let immutable = 2;
    let mut mutable = 2;

    println!("{}", immutable);
    mutable += 1;
    println!("{}", mutable);

    {
        let in_cope = 2;
        println!("this is in cope {}", in_cope);
    }

    let mut shadowing = 1;
    {
        println!("shadowing int {}", shadowing);

        let shadowing = "abc";

        println!("shadowing in cope {}", shadowing);
    }

    shadowing += 1;

    println!("this is in cope {}", shadowing);
    

    let a_binding;

    a_binding = 1;
    
    println!("{}", a_binding);

    let mut immutable_data = 2;
    {
        let immutable_data = immutable_data;
        // immutable_data = 3;
        println!("{}", immutable_data)
    }
    immutable_data = 3;
}
