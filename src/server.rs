use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("cliente conectado desde. {}", peer);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(msg) => println!("[{}] {} ", peer, msg),
            Err(_) => break,
        }
    }
    println!("cliente desconectado: {}", peer);
}

pub fn run_server() {
    let listener = TcpListener::bind(" ").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("conexion fallida {e}");
            }
        }
    }
}
