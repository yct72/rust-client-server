use std::net::TcpListener;
use std::net::TcpStream;
use clap::Parser;
use std::io::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    address: String,
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 512];
    stream.read(&mut buf)?;
    println!("Request from client: {}", String::from_utf8_lossy(&buf));
    
    let msg = b"Write from server.";
    stream.write(msg)?;
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Starting server..."); 
    let listener = TcpListener::bind(args.address)?;
    
    for stream in listener.incoming() {
        handle_connection(stream.unwrap())?;
    }
    
    Ok(())
}
