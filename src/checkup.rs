use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

#[path = "./request.rs"] mod request;
#[path = "./response.rs"] mod response;

pub fn send_checkup(target_address:&str, target_port:&str) {

    let mut target = format!("{}:{}", target_address, target_port);
    if target_address == "localhost" { target = format!("127.0.0.1:{}", target_port); }

    let perfect_requests = [
        (b"this is a test".to_vec(), "no response"),
        (b"GET / HTTP/1.1\n\n\n".to_vec(), "good response")
    ];

    for request in perfect_requests {

        let target = target.clone();

        // All Good
        thread::spawn(move || {
            run_route(&target.to_string(), request.0, request.1)        
        });

    }
    // stream.read(&mut [0; 128]);

}

pub fn run_route(target:&str, request:Vec<u8>, response:&str) {

    if send_checkup_route(&target, request) != response { 

        println!("Unexpected");

    }

}

pub fn send_checkup_route(target:&str, request:Vec<u8>) -> String {

    let mut stream = TcpStream::connect(target).expect("Error Connecting to the Server!");

    stream.write(&request[..]).unwrap();
    
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).expect("Reading Response Failed.");

    if buffer.len() == 0 { return "no response".to_string(); }

    // println!("RESPONSE: {}", buffer);

    // Parse the Response
    return "good response".to_string();
    
}