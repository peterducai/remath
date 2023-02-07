//use crate::engine;
use crate::math;

pub struct Group {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Group {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

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