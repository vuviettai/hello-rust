use crate::pokemon::{ReadRequest, ReadResponse, WriteRequest, WriteResponse, p2p_server::P2p};
use tonic::{Request, Response, Status};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::mpsc::channel;

use crate::node::*;

//use libp2p::request_response::channel;
//use hello_rust::reqchan::{channel, RequestSender, ResponseReceiver};
//use crate::reqchan::RequestSender;

pub struct PokemonP2p {
    //pub sender : RequestSender<Commands, Result<Responses, Errors>>,
    pub sender: UnboundedSender<Commands>,
    //pub sender: Sender<Commands>,
    //pub receiver: UnboundedReceiver<Responses>,
}
/*
impl PokemonP2p {
    pub fn getResponse(&mut self) -> Responses {
        match self.receiver.blocking_recv() {
            None => Responses::Error(),
            Some(res) => res
        }
    }
}
*/
 // implementing rpc for service defined in .proto
 #[tonic::async_trait]
 impl P2p for PokemonP2p {
     // our rpc impelemented as function

     async fn get_by_key(&self, request:Request<ReadRequest>) ->Result<Response<ReadResponse>,Status>{
         let req_key = request.get_ref().key.clone();
         //let pokemon_content = String::from("test content");
         let (response_sender, response_receiver) = channel();
         match self.sender.send(Commands::GetPokemon(req_key, response_sender)) {
             Ok(_) => {
                 let pokemon_content = match response_receiver.recv() {
                     Ok(res) => {
                         match res {
                             Responses::GotPokemon(content) => content,
                             Responses::Success() => String::from("{}"),
                             Responses::Error() => String::from("{}")
                         }
                     }
                     Err(_) => String::from("{}")
                 };
                 Ok(Response::new(ReadResponse {
                     // reading data from request which is awrapper around our SayRequest message defined in .proto
                     key: format!("received: {}", request.get_ref().key),
                     value: pokemon_content
                 }))
             }
             Err(_) => {
                 Ok(Response::new(ReadResponse {
                     // reading data from request which is awrapper around our SayRequest message defined in .proto
                     key: format!("received: {}", request.get_ref().key),
                     value: String::from("Error")
                 }))
             }
         }

     }
     async fn put_value(&self,request:Request<WriteRequest>)->Result<Response<WriteResponse>,Status>{
         let ref_key = request.get_ref().key.clone();
         match self.sender.send(Commands::PutPokemon(ref_key, request.get_ref().value.clone())) {
             Ok(_) => {
                 Ok(Response::new(WriteResponse {
                     // reading data from request which is awrapper around our SayRequest message defined in .proto
                     key: format!("put pokemon: {}, {}",request.get_ref().key, request.get_ref().value),
                     result : 0,
                     message:String::from("Successful")
                 }))
             }
             Err(_) => {
                 Ok(Response::new(WriteResponse {
                     // reading data from request which is awrapper around our SayRequest message defined in .proto
                     key: format!("put pokemon: {}, {}",request.get_ref().key, request.get_ref().value),
                     result : 0,
                     message:String::from("Error")
                 }))
             }
         }
     }
 }
