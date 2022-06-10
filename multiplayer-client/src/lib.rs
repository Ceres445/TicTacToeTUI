use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client { stream }
    }

    pub fn read_line(&mut self) -> String {
        let mut buffer = [0; 1024];
        self.stream.read(&mut buffer).unwrap();
        from_utf8(&buffer).unwrap().to_string().trim_matches(char::from(0)).to_string()
    }

    pub fn write_line(&mut self, line: &str) {
        self.stream.write(line.as_bytes()).unwrap();
    }
}
// fn main() {
//     match TcpStream::connect("localhost:3333") {
//         Ok(mut stream) => {
//             println!("Successfully connected to server in port 3333");

//             let msg = b"Hello!";

//             stream.write(msg).unwrap();
//             println!("Sent Hello, awaiting reply...");

//             let mut data = [0 as u8; 6]; // using 6 byte buffer
//             match stream.read_exact(&mut data) {
//                 Ok(_) => {
//                     if &data == msg {
//                         println!("Reply is ok!");
//                     } else {
//                         let text = from_utf8(&data).unwrap();
//                         println!("Unexpected reply: {}", text);
//                     }
//                 }
//                 Err(e) => {
//                     println!("Failed to receive data: {}", e);
//                 }
//             }
//         }
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
//     println!("Terminated.");
// }



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_client() {

        let mut client = Client::new(TcpStream::connect("localhost:3333").unwrap());
        client.write_line("Hello!");
        let reply = client.read_line();
        assert_eq!(reply, "Hello!");
    }

}
