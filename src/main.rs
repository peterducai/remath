// use std::io::prelude::*;
use std::net::TcpListener;

use remath::handler::handle_route;
use remath::threader::ThreadPool;
// use remath::astro::planet_mars::Mars;
// use remath::math::tensor::tensor;

//extern crate base64;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("listening at http://0.0.0.0:7878");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_route(stream);
        });
    }

    println!("Shutting down.");
}


