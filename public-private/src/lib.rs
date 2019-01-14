#![allow(dead_code)]

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function(); invalid -> function is private
    outermost::inside::inner_function();
    //outermost::inside::secret_function(); invalid -> function is private
}