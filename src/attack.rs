use crate::Target;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[path = "./request.rs"]
mod request;
#[path = "./response.rs"]
mod response;

pub fn send_attack(target: &Target) {
    let target = if target.host == "localhost" {
        format!("127.0.0.1:{}", target.port)
    } else {
        format!("{}:{}", target.host, target.port)
    };

    let request = b"GET / HTTP/1.1\r\nHeader: Testing\r\n\r\nBODY!".to_vec();

    for i in 1..3000 {
        let target = target.clone();
        let request = request.clone();

        // All Good
        thread::spawn(move || log_attack_route(&target, request, i));
    }
    // stream.read(&mut [0; 128]);
}

pub fn log_attack_route(target: &str, request: Vec<u8>, i: u32) {
    let response = send_attack_route(&target.to_string(), request);
    let resp_string: &str = &response.to_string();

    match resp_string {
        "good response" => {
            println!("✅ - {}", i);
        }
        _ => {
            println!("❌ - {}", i);
        }
    };
}

pub fn send_attack_route(target: &str, request: Vec<u8>) -> String {
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

    match stream.set_read_timeout(Some(Duration::new(5, 0))) {
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

    return "good response".to_string();
}
