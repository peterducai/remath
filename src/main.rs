use remath::ThreadPool;
// use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
// use std::thread;
// use std::time::Duration;
// use std::error::Error;
// use std::path::Path;
use std::fs::File;
// use std::io::ErrorKind;
// use std::io;
// use std::io::Read;


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

fn read_from_file(path: &String) -> Result<String, io::Error> {

    let mut f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(s) => s,
        Err(_) => continue,
    };

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
    let pth = [a, urlsub].join("").to_owned();
    let status_line = "HTTP/1.1 200 OK";
    //let notfound =  "HTTP/1.1 404 NOTFOUND";
    let notfound = String::from("HTTP/1.1 404 NOTFOUND");

    //let (status_line, path) = ("HTTP/1.1 200 OK", pth);
// ("HTTP/1.1 404 NOT FOUND", "404.html")
    
    println!("reading file {}", pth);

    // let f = File::open(pth);



    let mut contents = read_from_file(&pth).unwrap();

    //let mut contents = fs::read_to_string(pth).unwrap();

    if contents == "" {
        contents = String::from("HTTP/1.1 404 NOTFOUND");
    }

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
