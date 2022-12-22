
use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;
    use std::net::TcpStream;
    fn main() {
        println!("Tentative de connexion au serveur...");
        let stream = TcpStream::connect("127.0.0.1:7878");
        match stream {
            Ok(mut stream) => {
                println!("Connexion au serveur réussie !");


                // "{\"Subscribe\":{\"name\":\"Richbell\"}}";
                if let Ok(message) = serde_json::to_string( "Hello") {
                    let buf = message.as_bytes();
                    let n = buf.len() as u32;
                    let mut buf_n = n.to_be_bytes(); //to_le_bytes
                    stream.write(&buf_n).unwrap();// taille du message envoyer au server
                    stream.write(&buf).unwrap();

                    loop {

                        stream.read_exact(&mut buf_n).unwrap(); // on attend la réponse du server
                        let n = u32::from_be_bytes(buf_n);
                        let mut buf = Vec::<u8>::new();
                        buf.resize(n as usize, 0);
                        let s = stream.read(&mut buf).expect("Cannot read");
                        let msg = String::from_utf8_lossy(&buf);

                        println!("Receive message {}",msg);
                    }
                }


            }
            Err(e) => {
                println!("La connexion au serveur a échoué : {}", e);
            }
        }
    }
