use std::io::prelude::*;
use std::net::{TcpListener, TcpStream}; // Agregamos libreria TcpStream y TcpStream

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap(); // Escuchar la dirección donde nos vamos a conectar

    for stream in listener.incoming() {
                let stream = stream.unwrap();

                handle_connection(stream);
            }


}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
