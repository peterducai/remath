use remath::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::error::Error;
use std::path::Path;
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_route(stream);
        });
    }

    println!("Shutting down.");
}


fn handle_route(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    

    let getc = String::from_utf8_lossy(&buffer[..]);
    let suburl: Vec<&str> = getc.split_whitespace().collect();
    // println!("{:?}", suburl);
    println!("URL -> {}",suburl[1]);
    let mut urlsub = suburl[1];
    // ["foo", "bar", "baz"] 
    if urlsub == "/" {
        urlsub = "/index.html";
    }

    let mut a = "static";
    let pth = [a, urlsub].join("");
    let status_line = "HTTP/1.1 200 OK";
    //let notfound =  "HTTP/1.1 404 NOTFOUND";
    let notfound = String::from("HTTP/1.1 404 NOTFOUND");

    //let (status_line, path) = ("HTTP/1.1 200 OK", pth);
// ("HTTP/1.1 404 NOT FOUND", "404.html")
    
    println!("reading file {}", pth);

    // let f = File::open(pth);

    // let f = match f {
    //     Ok(file) => file,
    //     //Err(error) => panic!("Problem opening the file: {:?}", error),
    //     //Err(error) => (status_line, pth) = ("HTTP/1.1 404 NOTFOUND", pth),
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

        let pth2 = pth;

    let contents = fs::read_to_string(pth2).unwrap();

    let ct = match contents {
        Ok(bb) => bb,
        //Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => notfound,
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        ct.len(),
        ct
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
