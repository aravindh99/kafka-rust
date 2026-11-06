#![allow(unused_imports)]

use std::io::Write;
use std::net::TcpListener;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");


    let message_size = "00 00 00 02".to_string();
    let correlation_id = "00 00 00 07".to_string();

     let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

     for stream in listener.incoming() {
       match stream {
           Ok(mut stream) => {
                println!("accepted new connection");
               let _ = stream.write_all(&[0,0,0,0,0,0,0,7]);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
