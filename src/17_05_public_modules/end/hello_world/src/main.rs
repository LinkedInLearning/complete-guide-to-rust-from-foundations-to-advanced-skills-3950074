fn main() {
    hello::english();
    hello::casual::english();
}

mod hello {
    pub fn english() {
        println!("hello");
        spanish();
    }

    fn spanish() {
        println!("hola");
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
            super::spanish();
        }
    }
}
