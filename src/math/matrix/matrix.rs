//use crate::engine;
pub mod matrix;

pub fn matrix_public_function() {
    println!("matrix func `public_function()`");
    //engine::get_version();
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}