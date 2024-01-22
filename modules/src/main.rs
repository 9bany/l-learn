pub mod test_m {
    fn private_function() {
        println!("testm::private_function");
    }

    pub fn function() {
        private_function();
        println!("testm::function");
    }

    pub mod nested {
        pub fn function() {
            private_function();
            println!("test_m::nasted::function");
        }

        fn private_function() {
            println!("test_m::nasted::private_function");
        }
        
        pub(in crate::test_m) fn just_pub_in_test_m() {
            println!("test_m::nested::just_pub_in_another_mod");
        }

        pub(self) fn pub_function_in_nasted() {
            println!("test_m::nasted::pub_function_in_nasted");
        }

        pub(super) fn pub_function_in_parent() {
            pub_function_in_nasted();
            println!("test_m::nested::pub_function_in_parent");
        }
    }

    pub fn call_public_function_in_test_m() {
        nested::function();
        nested::just_pub_in_test_m();
        nested::pub_function_in_parent();
    }
    
    mod pri_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("test_m::pri_nested::function");
        }
        
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("test_m::pri_nested::restricted_function");
        }
    }

}

fn main() {
    test_m::function();
}
