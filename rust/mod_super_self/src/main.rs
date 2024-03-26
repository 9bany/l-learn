#[allow(dead_code)]
fn function() {
    println!("function");
}

mod cool {
    #[allow(dead_code)]
    pub fn function() {
        println!("cool::function");
    }
}

mod m {
    #[allow(dead_code)]
    fn function() {
        println!("m::function");
    }

    mod cool {
        #[allow(dead_code)]
        pub fn function() {
            println!("m::cool::function");
        }
    }

    pub fn call_direct() {
        function();
        crate::cool::function();
        self::function();
        super::function();
    }
}

fn main() {
    crate::m::call_direct();
}
