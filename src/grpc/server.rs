use tonic::{transport::Server, Request, Response, Status};
use hello_rust::pokemon::p2p_server::{P2p, P2pServer};
use hello_rust::pokemon::{ReadResponse, ReadRequest};
use hello_rust::pokserver::PokemonP2p;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "0.0.0.0:50001".parse().unwrap();
    // creating a service
    let pokemon_p2p = PokemonP2p::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(P2pServer::new(pokemon_p2p))
        .serve(addr)
        .await?;
    Ok(())
}
/*
// defining a struct for our service
#[derive(Default)]
pub struct MySay {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Say for MySay {
    // our rpc impelemented as function
    async fn send(&self,request:Request<SayRequest>)->Result<Response<SayResponse>,Status>{
        // returning a response as SayResponse message as defined in .proto
        Ok(Response::new(SayResponse{
            // reading data from request which is awrapper around our SayRequest message defined in .proto
            message:format!("hello {}",request.get_ref().name),
        }))
    }
}
 */
