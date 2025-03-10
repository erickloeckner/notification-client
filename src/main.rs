use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let location = env::args()
        .nth(1)
        .unwrap_or("".to_string());
    
    let command_string = env::args()
        .nth(2)
        .unwrap_or("0".to_string());
    let command = u8::from_str_radix(&command_string, 10).unwrap_or(0);

    let value_string = env::args()
        .nth(3)
        .unwrap_or("0".to_string());
    let value = u8::from_str_radix(&value_string, 10).unwrap_or(0);

    let message = [0, 0, 0, command, value, 255, 255, 255];
    let mut stream = TcpStream::connect(&location)?;
    stream.write(&message)?;

    Ok(())
}
