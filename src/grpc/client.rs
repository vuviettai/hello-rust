mod pokemon {
    tonic::include_proto!("pokemon");
}
use pokemon::p2p_client::P2pClient;
use pokemon::{ReadRequest, WriteRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //put client
    let put_channel = tonic::transport::Channel::from_static("http://[::1]:50051").connect().await?;
    //let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50001").connect().await?;
    // creating gRPC client from channel

    let mut put_client = P2pClient::new(put_channel);

    //Put pokemon
    let put_request = tonic::Request::new(
        WriteRequest {
            key:String::from("pokemon_name"),
            value: String::from("pokemon_content")
        },
    );
    let put_response = put_client.put_value(put_request).await?.into_inner();
    println!("RESPONSE={:?}", put_response);
    let put_request = tonic::Request::new(
        WriteRequest {
            key:String::from("pokemon_name2"),
            value: String::from("pokemon_content2")
        },
    );
    let put_response = put_client.put_value(put_request).await?.into_inner();
    println!("RESPONSE={:?}", put_response);

    let read_channel = tonic::transport::Channel::from_static("http://[::1]:50052").connect().await?;
    let mut read_client = P2pClient::new(read_channel);
    // read pokemon content
    let read_request = tonic::Request::new(
        ReadRequest {
            key:String::from("pokemon_name")
        },
    );
    // sending request and read_client for response
    let read_response = read_client.get_by_key(read_request).await?.into_inner();
    println!("RESPONSE={:?}", read_response);

    Ok(())
}