use std::io::prelude::*;
use std::net::{TcpListener, TcpStream}; // Agregamos libreria TcpStream y TcpStream

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap(); // Escuchar la dirección donde nos vamos a conectar

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("¡Nuevo cliente!");
            }
                Err(e) => {
                    println!("Conexión fallida")
                }
        }
    }

}
