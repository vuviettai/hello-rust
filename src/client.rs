use std::{error::Error, task::{Context, Poll}};
use async_std::{io, task};
use futures::prelude::*;
use tonic::{self, transport::Channel};
use env_logger;
mod pokemon;
use pokemon::{ReadRequest, ReadResponse, WriteRequest, p2p_client::P2pClient};
use serde_json::Result as JsonResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    color: String,
    eye_num: u8,
    nose_num: u8,
    mouth_num: u8,
}
struct GrpcClient {
    pub value : Option<P2pClient<Channel>>
}
impl GrpcClient {
    pub fn set_client(&mut self, client : P2pClient<Channel>) {
        self.value = Some(client);
    }
    //pub fn get_mut_value(&mut self) -> &Option<P2pClient<Channel>> {
    //    &self.value
    //}
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    env_logger::init();
    println!("Hello p2p client");
    let mut client = GrpcClient {value : None};
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match stdin.try_poll_next_unpin(cx)? {
                Poll::Ready(Some(line)) => handle_input_line(&mut client, line),
                Poll::Ready(None) => panic!("Stdin closed"),
                Poll::Pending => break
            }
        }
        Poll::Pending
    }))
    //let _listener = task::block_on(init_client()).unwrap();
    /*
    match listener {
       Ok(()) => (),
       Err(error) => panic!("Problem while init node {:?}", error)
    }
    */
}
fn handle_input_line(client : &mut GrpcClient, line: String) {
    let mut args = line.split(" ");
    //let v: Vec<&str> = line.split(" ").collect();
    //println!("{:?}", v);
    match args.next() {
        Some("init") => {
            match args.next() {
                Some(val) => {
                    let port: u16 = val.parse().unwrap();
                    let uri = format!("http://127.0.0.1:{:?}", port);
                    match Channel::from_shared(uri) {
                        Ok(endpoint) => {
                            match task::block_on(endpoint.connect()) {
                                Ok(channel) => {
                                    let p2p_client = P2pClient::new(channel);
                                    client.set_client(p2p_client);
                                    println!("Inited client with port {:?}", port);
                                }
                                _ => eprintln!("Cannot connect with given port")
                            };
                        }
                        _ => eprintln!("Inited client with port {:?}", port)
                    }
                },
                None => {
                    eprintln!("Expected port");
                    return;
                }
            }
        }
        Some("get") => {
            let key = {
                match args.next() {
                    Some(key) => key,
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            match client.value.as_mut(){
                Some(p2p_client) => {
                    let read_request = tonic::Request::new(
                        ReadRequest {
                            key:String::from(key)
                        },
                    );
                    match task::block_on(p2p_client.get_by_key(read_request)) {
                        Ok(res) => {
                            let response : ReadResponse = res.into_inner();
                            let pokemon : Pokemon = serde_json::from_str(&response.value).unwrap();
                            println!("RESPONSE={:?}", pokemon)
                        },
                        Err(err) => eprintln!("RESPONSE={:?}", err),
                    }
                }
                None => {
                    eprintln!("init client first");
                    return;
                }
            }
        }
        Some("put") => {
            let key = {
                match args.next() {
                    Some(key) => key,
                    None => {
                        eprintln!("Expected key");
                        return;
                    }
                }
            };
            let vec: Vec<&str> = args.collect();
            if vec.len() > 0 {
                let str_value: String = vec.join(" ");
                let result : JsonResult<Pokemon> = serde_json::from_str(str_value.as_str());
                match result {
                    Ok(pokemon) => {
                        match client.value.as_mut(){
                            Some(p2p_client) => {
                                let req_value = serde_json::to_string(&pokemon).unwrap();
                                let put_request = tonic::Request::new(
                                    WriteRequest {
                                        key:String::from( key),
                                        value: req_value
                                    },
                                );
                                let response = task::block_on(p2p_client.put_value(put_request));
                                match response {
                                    Ok(res) => println!("RESPONSE={:?}", res.into_inner()),
                                    Err(err) => eprintln!("RESPONSE={:?}", err),
                                }

                            }
                            None => {
                                eprintln!("init client first");
                                return;
                            }
                        }
                    },
                    _ => {
                        eprintln!("Invalid json format");
                        return;
                    }
                }
            } else {
                eprintln!("expected pokemon value in json format");
            }

        },
        _ => {
            eprintln!("expected init, get or put ");
        }
    }
}


