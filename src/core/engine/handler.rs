use std::net::TcpStream;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::fs;

pub fn handle_route(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    let getc = String::from_utf8_lossy(&buffer[..]);
    let suburl: Vec<&str> = getc.split_whitespace().collect();
    // println!("{:?}", suburl);
    println!("URL -> {}",suburl[1]);
    let mut urlsub = suburl[1];
    if urlsub == "/" {
        urlsub = "/index.html";
    }

    let a = "docs"; // URL/static + urlsub
    let pth = [a, urlsub].join("").to_owned();
    let response = file_to_response(&pth);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn file_to_response(path: &str) -> String {
    if Path::new(&path).exists() {
        println!("reading file {}", &path);

        // if path.contains(".png") {
        //     let mut part = "data:image/png;base64,";
        //     let cts = fs::read_to_string(&path);
        //     //let encoded = encode(cts);
        //     let xxx = [part, cts].join("").to_owned();
        //     let response = format!(
        //     "{}\r\nContent-Length: {}\r\n\r\n{}",
        //     "HTTP/1.1 200 OK",
        //     xxx.len(),
        //     xxx
        // );
        // return response;
        // } else {

            //if png, then DONT read to string
            // let mut content =  Cursor::new(response.bytes().await?);
            // copy(&mut content, &mut dest)?;

            let cts = fs::read_to_string(&path).unwrap();
            let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            "HTTP/1.1 200 OK",
            cts.len(),
            cts
        );
        return response;
        // }

    }  else {
        println!("DONT EXIST {}", path);
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            "HTTP/1.1 404 NOT FOUND",
            0,
            ""
        );
        return response;
    }
}