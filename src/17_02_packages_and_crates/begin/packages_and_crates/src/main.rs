/************************************
 * packages_and_crates binary crate *
 ************************************/

mod some_module;
use my_project; // library crate for the packages_and_crates package

fn main() {
    println!("Running the my_project executable.");
    some_module::mod_func();
    my_project::lib_func();
}
