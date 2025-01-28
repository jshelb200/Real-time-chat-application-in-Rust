use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server!");

    let mut input = String::new();
    loop {
        print!("Enter message: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;

        stream.write_all(input.as_bytes())?;

        if input.trim() == "exit" {
            println!("Closing connection.");
            break;
        }

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;
        println!("Server: {}", response.trim());
    }

    Ok(())
}
