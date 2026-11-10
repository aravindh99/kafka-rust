#![allow(unused_imports)]

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()>  {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");



     let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

     for stream in listener.incoming() {
       match stream {
           Ok(mut stream) => {
               println!("accepted new connection");
               loop {
                   let mut buffer = [0u8; 4];
                   stream.read_exact(&mut buffer)?;
                   let request_size = u32::from_be_bytes(buffer);
                   let mut request_buffer = vec![0u8; request_size as usize];

                   stream.read_exact(&mut request_buffer)?;

                   let message_size: i32 = 19;
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
                   stream.write_all(&message_size.to_be_bytes())?;
                   stream.write_all(&correlation_id)?;
                   stream.write_all(&error_code.to_be_bytes())?;
                   stream.write_all(&[02])?;
                   stream.write_all(&req_api_key.to_be_bytes())?;
                   stream.write_all(&[00, 00])?;
                   stream.write_all(&[00, 04])?;
                   stream.write_all(&[00])?;
                   stream.write_all(&[00, 00, 00, 00])?;
                   stream.write_all(&[00])?;
               }
           }
               Err(e) => {
                   println!("error: {}", e);
           }

        }
    }
    Ok(())

}
