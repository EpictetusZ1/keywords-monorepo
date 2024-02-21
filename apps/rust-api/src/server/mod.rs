use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.fill_buf().unwrap();
    // print the entire request:
    println!("Request: {}", String::from_utf8_lossy(request_line));


    // Check if this is a preflight request and handle accordingly
    if request_line.starts_with(b"OPTIONS ") {
        let response = "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, GET, OPTIONS\r\nAccess-Control-Allow-Headers: X-Requested-With, Content-Type, Accept\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // read response:

        // This is where you'd normally handle your POST requests
        let response_base = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/plain\r\nContent-Length: ";
        // Two response types, good and bad
        // attempt to add to database
        let response = format!("{}{}\r\n\r\n", response_base, "0");

        stream.write_all(response.as_bytes()).unwrap();
    }
}



// write prepare data for database entry: