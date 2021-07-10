extern crate serde;
use serde_json::{Result as JsonResult, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    color: String,
    eye_num: u8,
    nose_num: u8,
    mouth_num: u8,
}
fn main() {
    let value : String = String::from("{\"color\":\"blue\",\"eye_num\":1,\"nose_num\":2,\"mouth_num\":3}");
    let pokemon : Pokemon = serde_json::from_str(value.as_str()).unwrap();
    /*
    let pokemon = Pokemon {
        color : String::from("blue"),
        eye_num : 1,
        nose_num : 2,
        mouth_num : 3
    };
     */
    println!("{:?}", pokemon);
    let pokemon_str = serde_json::to_string(&pokemon).unwrap();
    println!("{:?}",pokemon_str);
}