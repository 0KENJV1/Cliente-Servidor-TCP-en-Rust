mod server;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;

fn run_client() {
    match TcpStream::connect(" ") {
        Ok(mut stream) => {
            println!("conectado al servidor. Escribe mensajes: ");
            let stdin = io::stdin();

            for line in stdin.lock().lines() {
                let msg = line.unwrap();

                if msg == "/salir" {
                    break;
                }
                writeln!(stream, "{}", msg).unwrap()
            }
        }
        Err(e) => {
            println!("no se pudo conectar: {}", e);
        }
    }
}
fn main() {
    println!("modo? (s => servidor, c => cliente)");

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();

    match mode.trim() {
        "s" => server::run_server(),
        "c" => run_client(),
        _ => println!("modo no reconocido"),
    }
}
