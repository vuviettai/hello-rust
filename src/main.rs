mod pokemon;
//mod pokserver;
use async_std::task;
use env_logger;
use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;
use rand::prelude::*;
use std::{error::Error, thread};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use tokio::sync::mpsc;
use hello_rust::node;
use hello_rust::pokemonrpc::{PokemonRpcImpl, Rpc};


#[async_std::main]
//#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut rng = rand::thread_rng();
    //let addrs = ["[::1]:50051", "[::1]:50052"];
    //let p2p_addrs = ["/ip4/0.0.0.0/tcp/40051", "/ip4/0.0.0.0/tcp/40052"];
    //Grpc server
    let port: u16;
    if let Some(arg) = std::env::args().nth(1) {
        port = arg.parse()?;
    } else {
        port = rng.gen_range(50051..50100)
    }
    println!("Rpc port: {}", port);
    let grpc_socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let (request_sender, request_receiver) = mpsc::unbounded_channel();
    let jsonrpc_handle = thread::spawn(move || {
        let mut io = IoHandler::default();
        let pokemon_service = PokemonRpcImpl {
            sender : request_sender
        };
        io.extend_with(pokemon_service.to_delegate());
        /*
        io.add_method("put",move |params: Params| async {
            let pokemon: model::Pokemon = params.parse().unwrap();
            let name = &pokemon.name.clone();
            Ok(Value::String(format!("hello, {}", name)))
        });
        io.add_method("get",|params: Params| async move {
            let pokemon: model::Pokemon = params.parse().unwrap();
            //let name = &pokemon.name.clone();
            //get_sender.send(node::Commands::getPokemon(name));
            Ok(Value::String(format!("hello, {}", pokemon.name)))
        });
        */
        let server = ServerBuilder::new(io)
            .threads(3)
            .start_http(&grpc_socket)
            .unwrap();
        server.wait();
    });
    /*
    tokio::spawn(async move {
        let pokemon_p2p = PokemonP2p { sender : request_sender };
        let serve = Server::builder()
            .add_service(P2pServer::new(pokemon_p2p))
            .serve(grpc_socket);
        if let Err(e) = serve.await {
            eprintln!("Error = {:?}", e);
        }
    });
    */
    let p2p_port= rng.gen_range(40001..40100);
    let p2p_addr = format!("{}{}", "/ip4/0.0.0.0/tcp/", p2p_port);
    let p2p_handle = thread::spawn(move || {
        let swarm = task::block_on(node::create_swarm(&p2p_addr)).unwrap();
        let p2p_node = node::init_node(swarm, request_receiver);
        task::block_on(p2p_node).unwrap();
    });

    p2p_handle.join().unwrap();
    jsonrpc_handle.join().unwrap();
    Ok(())
}
