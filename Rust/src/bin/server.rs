use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("New connection from {}", peer_addr);

    let mut input = String::new();

    loop {
        // Crée un nouveau lecteur pour chaque itération
        {
            let mut reader = BufReader::new(&stream);
            input.clear();
            let bytes_read = reader.read_line(&mut input)?;
            if bytes_read == 0 {
                println!("Client {} disconnected.", peer_addr);
                break;
            }
            println!("Message: {}", input.trim());
        }

        // Écriture dans le flux après avoir libéré le lecteur
        stream.write_all(input.as_bytes())?;
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Erreur de gestion client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection echoue: {}", e),
        }
    }

    Ok(())
}
