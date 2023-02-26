use std::io::{Read, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe{
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
        Ok,
     Message(Message)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    SubscribeError(SubscribeError)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer{
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge {

    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeOutput {
    pub path: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge)
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
                    // on attend la réponse du server
                   match  stream.read(&mut buf_n) {
                        Ok(r) => {
                            println!("Received {} bytes", r);
                            println!("Message du serveur -> {}", String::from_utf8_lossy(&buf_n[..]));
                        },
                        Err(e) => {
                            panic!("error parsing header: {e:?}");
                        }
                    };
                    let n = u32::from_be_bytes(buf_n);
                    let mut buf = Vec::<u8>::new();
                    buf.resize(n as usize, 0);
                    stream.read(&mut buf).expect("Cannot read");
                    let m = String::from_utf8(buf).expect("form utf 8 error");


                    let deserialized = serde_json::from_str::<ClientMessage>(&m);
                    match deserialized {
                        Ok(x) => {
                            println!("deserialized = {:?}", x);
                            match x {
                                ClientMessage::Welcome(w) => {
                                    let subscribe = Subscribe { name: "test_test".to_owned() };
                                    let message = ClientMessage::Subscribe(subscribe);
                                    let message_json = serde_json::to_string(&message).unwrap();
                                    println!("Envoi du message : {}", message_json);
                                    let buf = message_json.as_bytes();
                                    let n = buf.len() as u32;
                                    let buf_n = n.to_be_bytes();
                                    stream.write(&buf_n).unwrap();
                                    stream.write(&buf).unwrap();
                                }
                                ClientMessage::Subscribe(w)=> {
                                    println!("Subscribe");
                                    println!("Subscribe name = {}", w.name);
                                }
                                ClientMessage::SubscribeResult(w)=> {
                                    println!("SubscribeResult");
                                    println!("Subscribe name = {:?}", w);
                                }
                                ClientMessage::PublicLeaderBoard(bord) => {
                                    println!("learderboard");
                                }
                                ClientMessage::Challenge(c)=> {
                                    match c {
                                        Challenge::MD5HashCash(chalengHash) => {
                                            //TODO mettre le code du challenge ici
                                            println!("MD5HashCash");
                                        }
                                        Challenge::MonstrousMaze(_) => {}
                                    }
                                }
                            }
                        },
                        Err(e) =>  panic!("error parsing header: {:?}",e)
                    }


                    //println!("Receive message msg {}",m);
                    // send Subscribe message

                }
            }
        }
        Err(e) => { //Si c'est non
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}






