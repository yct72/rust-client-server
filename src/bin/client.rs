use std::net::TcpStream;
use std::io::*;

fn main() -> Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:7272")?;
    let msg = "Hello from client.".as_bytes();
    let mut response = [0; 100];

    stream.write(msg)?;
    stream.read(&mut response)?;
    println!("Read from server: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
