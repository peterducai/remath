// use std::io::prelude::*;

use remath::core::engine::handler::handle_route;
use remath::core::engine::threader::ThreadPool;
use remath::core::engine::version;
use std::net::{TcpListener, TcpStream};


//extern crate base64;

fn main() {
    version::version::get_version(2, 4);
    //matrix::matrix_public_function();
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("listening at http://0.0.0.0:7878 or http://localhost:7878");
    let pool = ThreadPool::new(4);



    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_route(stream);
        });
    }

    //println!("{ig.description}");
    println!("Shutting down.");

}


