//use crate::engine;
use crate::math;

pub struct Group {
    pub name: String,
    pub set: String,
    pub operation: String,
    pub content: String,
    pub latex: String
}

impl EmptyGroup for Group {
    fn init(&self) {
        self.latex = "$$ a \\otimes b = ab $$"
        self.content = "va⊗b is known as an outer product of a and b."
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.name, self.set, self.operation, self.content)
    }
}

pub fn math_group_get_version() {
    println!("matrix func `public_function()`");
    //engine::get_version();
    matrix::matrix_public_function();
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}