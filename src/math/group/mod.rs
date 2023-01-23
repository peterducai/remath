//use crate::engine;
use crate::math;

pub fn math_group_get_version() {
    println!("matrix func `public_function()`");
    //engine::get_version();
    math::matrix::matrix_public_function();
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}