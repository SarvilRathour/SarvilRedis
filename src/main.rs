use std::io::{BufReader,Write};
use std::net::TcpListener;
use std::net::TcpStream;
fn handle_connection(mut stream:TcpStream){
    let mut bufread=BufReader::new(&mut stream);
    stream.write_all("Ping".as_bytes());
}
fn main()->std::io::Result<()> {
    let listener=TcpListener::bind("127.0.0.1:6379")?;
    for stream in listener.incoming(){
        let stream=stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}
