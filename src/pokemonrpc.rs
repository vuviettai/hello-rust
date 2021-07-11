use jsonrpc_core::{Result, Params};
use jsonrpc_derive::rpc;
use std::sync::mpsc::channel;
use crate::model::{Commands, Pokemon, Responses};
use tokio::sync::mpsc::UnboundedSender;

#[rpc]
pub trait Rpc {
    /// Adds two numbers and returns a result
    #[rpc(name = "put")]
    fn put(&self, params : Params) -> Result<String>;
    #[rpc(name = "put_value")]
    fn put_value(&self, name : String, color: String, eye_num : u32, nose_num : u32, mouth_num : u32) -> Result<String>;
    #[rpc(name = "get")]
    fn get(&self, name:String) -> Result<Pokemon>;
}

pub struct PokemonRpcImpl {
    pub sender : UnboundedSender<Commands>
}
impl Rpc for PokemonRpcImpl {
    fn put(&self, params : Params) -> Result<String> {
        let pokemon: Pokemon = params.parse().unwrap();
        match self.sender.send(Commands::StorePokemon(pokemon)) {
            Ok(_) => {
                Ok(String::from("Ok"))
            }
            Err(_) => {
                Ok(String::from("Error"))
            }
        }
    }
    fn put_value(&self, name : String, color: String, eye_num : u32, nose_num : u32, mouth_num : u32) -> Result<String> {
        let pokemon = Pokemon {
            name, color, eye_num, nose_num, mouth_num
        };
        match self.sender.send(Commands::StorePokemon(pokemon)) {
            Ok(_) => {
                Ok(String::from("Ok"))
            }
            Err(_) => {
                Ok(String::from("Error"))
            }
        }
    }
    fn get(&self, name : String) -> Result<Pokemon> {
        let (response_sender, response_receiver) = channel();
        let key = name.clone();
        match self.sender.send(Commands::GetPokemon(key, response_sender)) {
            Ok(_) => {
                let pokemon = match response_receiver.recv() {
                    Ok(res) => {
                        match res {
                            Responses::GotPokemon(content) => {
                                let res : Pokemon = serde_json::from_str(&content).unwrap();
                                res
                            },
                            _ => Pokemon{
                                name,
                                color: "".to_string(),
                                eye_num: 0,
                                nose_num: 0,
                                mouth_num: 0
                            }
                        }
                    }
                    Err(_) => Pokemon{
                        name,
                        color: "".to_string(),
                        eye_num: 0,
                        nose_num: 0,
                        mouth_num: 0
                    }
                };
                Ok(pokemon)
            }
            Err(_) => {
                Ok(Pokemon{
                    name,
                    color: "".to_string(),
                    eye_num: 0,
                    nose_num: 0,
                    mouth_num: 0
                })
            }
        }

    }
}
/*
pub struct PokemonPut {
    pub sender : UnboundedSender<Commands>
}
impl RpcMethodSimple for PokemonPut {
    type Out = ();
    fn call(&self, params: Params) -> Self::Out {
        let pokemon: Pokemon = params.parse().unwrap();
        let name = &pokemon.name.clone();
        //put_sender.send(Commands::StorePokemon(pokemon));
        Ok(Value::String(format!("hello, {}", name)))
    }
}
*/