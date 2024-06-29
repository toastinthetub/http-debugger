use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn send_custom_request(ip: &str, port: u16, request_file: &str) -> io::Result<()> {
    let file = File::open(request_file)?;
    let reader = BufReader::new(file);
    let mut request = String::new();

    for line in reader.lines() {
        let line = line?;
        request.push_str(&line);
        request.push('\n');
    }

    let addr = format!("{}:{}", ip, port);

    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() {
    let ip = "172.233.158.174";
    let port = 8080;
    let request_file = "./request.txt";

    if let Err(e) = send_custom_request(ip, port, request_file) {
        eprintln!("Error: {}", e);
    }
}
