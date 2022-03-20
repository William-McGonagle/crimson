use crate::Target;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[path = "./request.rs"]
mod request;
#[path = "./response.rs"]
mod response;

pub fn send_checkup(target: &Target) {
    let target = if target.host == "localhost" {
        format!("127.0.0.1:{}", target.port)
    } else {
        format!("{}:{}", target.host, target.port)
    };

    let perfect_requests = [
        (b"this is a test".to_vec(), "no response"), // bad request
        (b"GET".to_vec(), "timeout"),                // bad request
        (b"GET ".to_vec(), "timeout"),               // bad request
        (b"GET /".to_vec(), "timeout"),              // bad request
        (b"GET / ".to_vec(), "timeout"),             // bad request
        (b"GET  ".to_vec(), "timeout"),              // bad request
        (b"GET  HTTP/1.1".to_vec(), "no response"),  // bad request
        (b"GET / HTTP/1.1".to_vec(), "timeout"),     // bad request
        (b"GET / HTTP/1.1\n\n\n".to_vec(), "good response"), // unix line endings
        (b"GET / HTTP/1.1\n\r\n\r\n\r".to_vec(), "good response"), // windows line endings
    ];

    for request in perfect_requests {
        // All Good
        // thread::spawn(move || {
        run_route(&target, request.0, request.1)
        // });
    }
    // stream.read(&mut [0; 128]);
}

pub fn run_route(target: &str, request: Vec<u8>, response: &str) {
    let actual = send_checkup_route(&target, request);

    if actual != response {
        println!("❌ Expected \"{}\" but got \"{}\"", response, actual);
    } else {
        println!("✅ Expected \"{}\" and got it", response);
    }
}

pub fn send_checkup_route(target: &str, request: Vec<u8>) -> String {
    let mut stream;

    match TcpStream::connect(target) {
        Ok(v) => {
            stream = v;
        }
        Err(v) => {
            return "error connecting".to_string();
        }
    }

    stream.write(&request[..]).unwrap();

    let mut buffer = String::new();

    match stream.set_read_timeout(Some(Duration::new(1, 0))) {
        Ok(v) => {}
        Err(v) => {
            return "timeout".to_string();
        }
    }

    match stream.read_to_string(&mut buffer) {
        Ok(v) => {}
        Err(v) => {
            return "timeout".to_string();
        }
    }

    if buffer.len() == 0 {
        return "no response".to_string();
    }

    // Parse the Response
    return "good response".to_string();
}
