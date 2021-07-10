use std::{error::Error, task::{Context, Poll}};
use async_std::task;
use futures::prelude::*;
use libp2p::kad::{
    AddProviderOk,
    Kademlia,
    KademliaEvent,
    PeerRecord,
    PutRecordOk,
    QueryResult,
    Quorum,
    Record,
    record::{Key,store::MemoryStore}
};
use libp2p::{identity, development_transport, PeerId, Swarm, NetworkBehaviour, mdns::{Mdns, MdnsConfig, MdnsEvent}, swarm::{NetworkBehaviourEventProcess}};
use tokio::sync::mpsc::UnboundedReceiver;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Errors {
    NoSuchPerson,
}
pub enum Commands {
    PutPokemon(String, String),
    GetPokemon(String, Sender<Responses>)
}
#[derive(Debug)]
pub enum Responses {
    Success(),
    Error(),
    GotPokemon(String)
}

// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
pub struct MdnsBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub sender: Option<Sender<Responses>>
}
impl MdnsBehaviour {
    pub fn set_sender(&mut self, sender : Sender<Responses>) {
        self.sender = Some(sender);
    }
}
impl NetworkBehaviourEventProcess<MdnsEvent> for MdnsBehaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                self.kademlia.add_address(&peer_id, multiaddr);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<KademliaEvent> for MdnsBehaviour {
    // Called when `kademlia` produces an event.
    fn inject_event(&mut self, message: KademliaEvent) {
        match message {
            KademliaEvent::QueryResult { result, .. } => match result {
                QueryResult::GetProviders(Ok(ok)) => {
                    for peer in ok.providers {
                        println!(
                            "Peer {:?} provides key {:?}",
                            peer,
                            std::str::from_utf8(ok.key.as_ref()).unwrap()
                        );
                    }
                }
                QueryResult::GetProviders(Err(err)) => {
                    eprintln!("Failed to get providers: {:?}", err);
                }
                QueryResult::GetRecord(Ok(ok)) => {
                    for PeerRecord { record: Record { key, value, .. }, ..} in ok.records {
                        let pokemon_content = String::from(std::str::from_utf8(&value).unwrap());
                        match &self.sender {
                            None => {},
                            Some(sender) => {
                                match sender.send(Responses::GotPokemon(pokemon_content)).unwrap() {
                                    _ => {}
                                }
                            }
                        };
                        /*
                        println!(
                            "Got record {:?} {:?}",
                            std::str::from_utf8(key.as_ref()).unwrap(),
                            std::str::from_utf8(&value).unwrap(),
                        );
                         */
                    }
                }
                QueryResult::GetRecord(Err(err)) => {
                    eprintln!("Failed to get record: {:?}", err);
                }
                QueryResult::PutRecord(Ok(PutRecordOk { key })) => {
                    println!(
                        "Successfully put record {:?}",
                        std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::PutRecord(Err(err)) => {
                    eprintln!("Failed to put record: {:?}", err);
                }
                QueryResult::StartProviding(Ok(AddProviderOk { key })) => {
                    println!("Successfully put provider record {:?}",
                             std::str::from_utf8(key.as_ref()).unwrap()
                    );
                }
                QueryResult::StartProviding(Err(err)) => {
                    eprintln!("Failed to put provider record: {:?}", err);
                }
                _ => {}
            }
            _ => {}
        }
    }
}

pub async fn create_swarm(addr: &String) -> Result<Swarm<MdnsBehaviour>, Box<dyn Error>>  {
// Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol.
    let transport = development_transport(local_key).await?;

    // Create a Kademlia behaviour.
    let store = MemoryStore::new(local_peer_id);
    let kademlia = Kademlia::new(local_peer_id, store);
    let mdns = task::block_on(Mdns::new(MdnsConfig::default()))?;
    let behaviour = MdnsBehaviour { kademlia, mdns, sender: None };
    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on(addr.parse()?)?;
    Ok(swarm)
}

pub async fn init_node (mut swarm : Swarm<MdnsBehaviour>,
                         mut request_receiver: UnboundedReceiver<Commands>) -> Result<(), Box<dyn Error>> {
    let mut listening = false;
    //let mut stdin = io::BufReader::new(io::stdin()).lines();
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        loop {
            match request_receiver.poll_recv(cx) {
                Poll::Ready(Some(commands)) => {
                    match commands {
                        Commands::PutPokemon(name, content) => {
                            println!("Receive pokemon name {:?} with content {:?}", name, content);
                            let key = Key::new(&name);
                            let record = Record {
                                key,
                                value: content.as_bytes().to_vec(),
                                publisher: None,
                                expires: None,
                            };
                            swarm.behaviour_mut().kademlia.put_record(record, Quorum::One).expect("Failed to store record locally.");
                            //Ok(Responses::Success())
                        }
                        Commands::GetPokemon(name, sender) => {
                            let key = Key::new(&name);
                            println!("Receive request for name {:?}", name);
                            swarm.behaviour_mut().set_sender(sender);
                            //sender.send(Responses::GotPokemon(String::from("fdasfds")));
                            swarm.behaviour_mut().kademlia.get_record(&key, Quorum::One);
                            //Ok(Responses::Success())
                        }
                    }
                },
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => break
            }
        }
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("{:?}", event),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => {
                    if !listening {
                        if let Some(a) = Swarm::listeners(&swarm).next() {
                            println!("Listening on {:?}", a);
                            listening = true;
                        }
                    }
                    break
                }
            }
        }
        Poll::Pending
    }))
}
