use remath::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("listening at http://127.0.0.1:7878");
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

    let mut a = "static"; // URL/static + urlsub
    let mut pth = [a, urlsub].join("").to_owned();
    let mut status_line = "HTTP/1.1 200 OK";
    //let found =  "HTTP/1.1 200 OK";
    let notfound = "HTTP/1.1 404 NOTFOUND";

    let mut contents = "data";
    // let mut contents = read_from_file(&pth).unwrap();

    let mut response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    

    if Path::new(&pth).exists() {
        println!("reading file {}", pth);
        let cts = fs::read_to_string(pth).unwrap();
        //contents = &cts;
        //fs::read_to_string(pth).unwrap();
        //status_line = notfound;
        //contents = &cts;
        response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            cts.len(),
            cts
        );
    }  else {
        println!("DONT EXIST {}", pth);
        response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    }

    

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
