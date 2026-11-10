#![allow(unused_imports)]

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()>  {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");



     let listener = TcpListener::bind("127.0.0.1:9092")?;

     for stream in listener.incoming() {
       match stream {
           Ok(stream) => {
               println!("accepted new connection");
               thread::spawn(move || {
                   if let Err(e) = handle_client(stream) {
                       eprintln!("Client error: {e}");
                   }
               });

           }
               Err(e) => {
                   println!("error: {}", e);
           }

        }
    }
    Ok(())

}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    loop {
        let mut buffer = [0u8; 4];

        match stream.read_exact(&mut buffer) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }

        let request_size = i32::from_be_bytes(buffer);
        let mut request_buffer = vec![0u8; request_size as usize];

        stream.read_exact(&mut request_buffer)?;

        let correlation_id = &request_buffer[4..8];
        let req_api_key = u16::from_be_bytes([
            request_buffer[0],
            request_buffer[1],
        ]);
        let req_api_version = u16::from_be_bytes([
            request_buffer[2],
            request_buffer[3],
        ]);
        let error_code: i16 = match req_api_version {
            0..=4 => 0,
            _ => 35,
        };
        let mut response = Vec::new();
        response.extend_from_slice(&[0, 0, 0, 0]);

        response.extend_from_slice(correlation_id);
        response.extend_from_slice(&error_code.to_be_bytes());
        response.extend_from_slice(&[02]);
        response.extend_from_slice(&req_api_key.to_be_bytes());
        response.extend_from_slice(&[00, 00]);
        response.extend_from_slice(&[00, 04]);
        response.extend_from_slice(&[00]);
        response.extend_from_slice(&[00, 00, 00, 00]);
        response.extend_from_slice(&[00]);


        let message_size = (response.len() - 4) as i32;
        response[0..4].copy_from_slice(&message_size.to_be_bytes());


        stream.write_all(&response)?;
    }

Ok(())
}
