use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Can't listene on this port :'(");
    println!("Server listening on port 3000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {            
                     handle_client(stream);
                });
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {

    let mut buffer = [0;1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            println!("Received request:\n{}", request);
            if let Some((method, path)) = parse_request(&request) {
                let response = match (method, path) {
                    ("GET", "/") => "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!",
                    ("GET", "/about") => "HTTP/1.1 200 OK\r\nContent-Length: 18\r\n\r\nAbout this server",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };
            
                stream.write_all(response.as_bytes()).expect("Couldn't hear you :(");
            } else {
                let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }
}

fn parse_request(request: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = request.splitn(3, ' ').collect();
    if parts.len() < 3 || !parts[0].starts_with("HTTP/") {
        return None;
    }
    Some((parts[0], parts[1]))
}
