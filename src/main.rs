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
               let mut buffer =[0u8; 1024];
               stream.read(&mut buffer)?;let message_size = u32::from_be_bytes([
                   buffer[0],
                   buffer[1],
                   buffer[2],
                   buffer[3],
               ]);
               let correlation_id = &buffer[8..12];
               let req_api_key = u16::from_be_bytes([
                   buffer[4],
                   buffer[5],
               ]);
               let req_api_version = u16::from_be_bytes([
                   buffer[6],
                   buffer[7],
               ]);
               let mut error_code: i16 = match req_api_version {
                   0..=4 => 0,
                   _=> 35,
               };
               stream.write_all(&message_size.to_be_bytes())?;
               stream.write_all(&correlation_id)?;
               stream.write_all(&error_code.to_be_bytes())?;
               stream.write_all(&[02])?;
               stream.write_all(&req_api_key.to_be_bytes())?;
               stream.write_all(&[00,00])?;
               stream.write_all(&[00, 04])?;
               stream.write_all(&[00])?;
               stream.write_all(&[00 ,00, 00, 00])?;
               stream.write_all(&[00])?;
           }
            Err(e) => {
                println!("error: {}", e);
            }

        }
    }
    Ok(())

}
