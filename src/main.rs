use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 4096] = [0; 4096];

    if stream.read(&mut buffer).is_ok() {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello World";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// Fonction principale
fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:111").unwrap();
    println!("Serveur rust démarré sur http://0.0.0.0:111");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erreur lors de la connexion : {}", e);
            }
        }
    }
}
