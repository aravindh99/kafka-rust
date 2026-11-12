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

        stream.read_exact(&mut buffer)?;

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

        if req_api_key == 18 {

            response.extend_from_slice(&error_code.to_be_bytes());
            response.extend_from_slice(&[03]);

            response.extend_from_slice(&18i16.to_be_bytes());
            response.extend_from_slice(&[00, 00]);
            response.extend_from_slice(&[00, 04]);
            response.extend_from_slice(&[00]);

            response.extend_from_slice(&75i16.to_be_bytes());
            response.extend_from_slice(&[00, 00]);
            response.extend_from_slice(&[00, 00]);
            response.push(0x00);

            response.extend_from_slice(&[00, 00, 00, 00]);
            response.push(0x00);

        } else if req_api_key == 75 {

            let mut pos = 8;
            let client_id_len = i16::from_be_bytes(
                request_buffer[pos..pos + 2].try_into().unwrap()
            );

            pos += 2;

            if client_id_len > 0 {
                pos += client_id_len as usize;
            }
            pos += 1;


            let topics_array_len = request_buffer[pos];
            pos += 1;

            assert_eq!(topics_array_len, 2);



            let topic_name_len = request_buffer[pos] as usize - 1;
            pos += 1;

            let topic_name = std::str::from_utf8(
                &request_buffer[pos..pos + topic_name_len]
            ).unwrap();


            response.push(0x00);
            response.extend_from_slice(&[0x00; 4]);
            response.push(0x02);
            response.extend_from_slice(&3i16.to_be_bytes());


            let topic_name = "foo";
            response.push((topic_name.len() + 1) as u8);
            response.extend_from_slice(topic_name.as_bytes());

            response.extend_from_slice(&[0x00; 16]);
            response.push(0x00);
            response.push(0x01);
            response.extend_from_slice(&0i32.to_be_bytes());
            response.push(0x00);
            response.push(0xff);
            response.push(0x00);







        }







        let message_size = (response.len() - 4) as i32;
        response[0..4].copy_from_slice(&message_size.to_be_bytes());


        stream.write_all(&response)?;
    }

Ok(())
}
