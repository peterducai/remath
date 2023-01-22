extern crate rustc_version_runtime;
use rustc_version_runtime::version;

pub fn get_version() {
    println!("-------------------------------------");
    println!("remath engine 0.0.1");
    println!("compiled using Rust {:?}", version());
    println!("-------------------------------------");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}