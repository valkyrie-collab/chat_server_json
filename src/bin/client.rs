extern crate chat_server_json;

use std::thread;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader, StdinLock, BufRead, Error};
use std::time::Duration;

fn get_server_response(mut stream_reader: TcpStream) {
    let mut buffer: [u8; 2048] = [0; 2048];

    loop {
        match stream_reader.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                let msg: String = String::from_utf8_lossy(&mut buffer[0..n]).to_string();

                println!("{}", msg);
            }
            Err(e) => {
                println!("Err: {}", e);
                break;
            }
        }
    }

}

fn send_server_response(mut stream_writer: TcpStream) {
    println!("Enter your username: ");

    let mut username: String = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    username = format!("Username: {}", username.trim());
    stream_writer.write_all(username.as_bytes()).unwrap();

    let reader: BufReader<StdinLock> = BufReader::new(std::io::stdin().lock());

    for line in reader.lines() {

        match line {
            Ok(msg) => {

                if let Err(e) = stream_writer.write_all(msg.as_bytes()) {
                    println!("Error in sending data to server...: {}", e);
                    
                    break;
                }

                stream_writer.flush().unwrap();
            }
            Err(_) => {
                println!("Error reading lines....");
                
                break;
            }
        }

    }

}

fn main() {
    let res_stream: Result<TcpStream, Error> = TcpStream::connect("127.0.0.7:8080");
    
    match res_stream {
        Ok(stream) => {
            println!("Connection successful...");
            
            let stream_two: TcpStream = stream.try_clone().unwrap();

            println!("Your connected to port 8080...");
            
            thread::sleep(Duration::from_millis(500));
            
            println!("Welcome! to the group chat..");

            thread::spawn(move || {
                get_server_response(stream);
            });

            send_server_response(stream_two);
        }
        Err(e) => {
            println!("cannot be connected to the server port 8080....{}", e);
        }
    }
    
}