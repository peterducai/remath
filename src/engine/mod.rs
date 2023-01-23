extern crate rustc_version_runtime;
use rustc_version_runtime::version;
// use cargo_metadata::MetadataCommand;

pub fn get_version() {
    println!("-------------------------------------");
    println!("remath engine 0.0.1");
    println!("compiled using Rust {:?}", version());
    println!("-------------------------------------");
    // let metadata = MetadataCommand::new()
    //     .exec()
    //     .expect("failed to get metadata");
    //
    // let dependency = metadata.packages
    //     .iter()
    //     .find(|package| package.name == "postgres")
    //     .expect("failed to find dependency");
    //
    // println!("{:#?}", dependency.features);
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}