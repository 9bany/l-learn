use deep::nested::{
    fucntion as another_function,
    function1,
};

mod deep {
    pub mod nested {
        #[allow(dead_code)]
        pub fn fucntion() {
            println!("deep::nested::function");
        }

        pub fn function1() {
            println!("deep::nested::function 1");
        }
    }
}

fn main () {
    another_function();
    function1();
}