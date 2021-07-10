# hello-rust project

## description.

Simple implementation for exercise: https://github.com/massbitprotocol/helloworld-rust-project

## Instruction

1. git clone https://github.com/vuviettai/hello-rust.git

2. cd hello-rust
3. cargo run

Execute 5 nodes p2p at ports /ip4/0.0.0.0/tcp/40051 -> /ip4/0.0.0.0/tcp/40055 and 5 grpc servers at ports [::1]:50051 -> [::1]:50055

4. cargo run --bin client

Execute client from new terminal and enter following command for testing: 

  4.1 init 50051

  4.2 put pokemon1 {"color": "blue", "eye_num": 1, "nose_num": 2, "mouth_num": 3}

  4.3 init 50052

  4.4 get pokemon1
