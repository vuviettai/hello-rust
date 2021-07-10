mod pokemon;
mod pokserver;
use std::{error::Error, thread};
use async_std::task;

use env_logger;
use hello_rust::node;
use pokemon::p2p_server::P2pServer;
use pokserver::PokemonP2p;
use tonic::transport::Server;

use tokio::sync::mpsc;

//use hello_rust::reqchan::channel;
//#[async_std::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    //let addrs = ["[::1]:50051", "[::1]:50052"];
    //let p2p_addrs = ["/ip4/0.0.0.0/tcp/40051", "/ip4/0.0.0.0/tcp/40052"];
    let mut p2p_handles = vec![];
    for i in 0..6 {
        let (request_sender, request_receiver) = mpsc::unbounded_channel();
        //Grpc server
        let grpc_addr = format!("{}{}", "[::1]:5005", i).parse()?;
        tokio::spawn(async move {
            let pokemon_p2p = PokemonP2p { sender : request_sender };
            let serve = Server::builder()
                .add_service(P2pServer::new(pokemon_p2p))
                .serve(grpc_addr);
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }
        });

        let p2p_addr = format!("{}{}", "/ip4/0.0.0.0/tcp/4005", i);
        let p2p_handle = thread::spawn(move || {
            let swarm = task::block_on(node::create_swarm(&p2p_addr)).unwrap();
            let p2p_node = node::init_node(swarm, request_receiver);
            task::block_on(p2p_node).unwrap();
        });
        p2p_handles.push(p2p_handle);
    }
    for handle in p2p_handles {
        handle.join().unwrap();
    }
    Ok(())
}
