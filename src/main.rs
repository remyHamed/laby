use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct S {
    subscribe: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    welcom: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeResult {
    #[serde(rename = "Ok")]
    ok: bool,
    #[serde(rename = "Err")]
    err: Option<SubscribeError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubscribeError {
    message: String,
}

fn main() { 
    println!("Tentative de connexion au serveur...");
    let stream = TcpStream::connect("127.0.0.1:7878"); //Connection entre serveur et nous
    match stream { //return result
        Ok(mut stream) => { //si c'est oui 
            println!("Connexion au serveur réussie !");

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
                    buf.resize(n as usize, 0); //taille du message reçu
                    let s = stream.read(&mut buf).expect("Cannot read");
                    let msg = String::from_utf8(buf).unwrap();
                    let w: String = serde_json::from_str(&message).unwrap();
                    println!("Receive message {:?}",w);
                    println!("Receive message {}",msg);

                    // envoyer un message Subscribe
                    let subscribe = Subscribe { name: "free_patato".to_string() };
                    let subscribe_message = serde_json::to_string(&subscribe).unwrap();
                    let buf = subscribe_message.as_bytes();
                    let n = buf.len() as u32;
                    let buf_n = n.to_be_bytes();
                    stream.write(&buf_n).unwrap(); // taille du message envoyé au serveur
                    stream.write(&buf).unwrap();
                    println("")
                    println!("Message Subscribe envoyé !");

                    let n = u32::from_be_bytes(buf_n);
                    let mut buf = vec![0u8; n as usize]; // créer un vecteur de taille n pour stocker les données
                    stream.read_exact(&mut buf).expect("Cannot read"); // lire exactement n octets dans le flux
                
                    // désérialiser le message reçu en tant que Value
                    let result: serde_json::Value = serde_json::from_slice(&buf).unwrap();
                
                    // vérifier si le message est une souscription réussie ou non
                    if let Some(ok) = result.get("ok").and_then(|ok| ok.as_bool()) {
                        if ok {
                            println!("Received message: SubscribeResult - Ok");
                        } else {
                            if let Some(error) = result.get("error").and_then(|error| error.as_str()) {
                                println!("Received message: SubscribeResult - Err({})", error);
                            } else {
                                println!("Received message: SubscribeResult - Err(unknown error)");
                            }
                        }
                    } else {
                        println!("Received message: unknown message");
                    }
                    
                }
            }
        }
        Err(e) => { //Si c'est non
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}






