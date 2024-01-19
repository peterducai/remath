// use std::io::prelude::*;

use remath::core::engine::handler::handle_route;
use remath::core::engine::threader::ThreadPool;
use remath::core::engine::version;
// use remath::math::group;
// // use remath::astro::planet_mars::Mars;
// // use remath::math2::tensor::tensor;
// use remath::math::complex;
//use remath::core::math::matrix;
//use remath::math::tensor;
use remath::quantum::identity_gate::identity_gate;


use openssl::ssl::{SslMethod, SslAcceptor, SslStream, SslFiletype};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    acceptor.set_certificate_chain_file("certs.pem").unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

    fn handle_client(stream: SslStream<TcpStream>) {
        // ...
    }
}

for stream in listener.incoming() {
match stream {
Ok(stream) => {
let acceptor = acceptor.clone();
thread::spawn(move || {
let stream = acceptor.accept(stream).unwrap();
handle_client(stream);
});
}
Err(e) => { /* connection failed */ }
}
}


//extern crate base64;

// fn main() {
//     version::version::get_version(2, 4);
//     //matrix::matrix_public_function();
//     let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
//     println!("listening at http://0.0.0.0:7878 or http://localhost:7878");
//     let pool = ThreadPool::new(4);
//
//
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//
//         pool.execute(|| {
//             handle_route(stream);
//         });
//     }
//
//     //println!("{ig.description}");
//     println!("Shutting down.");
//
// }


