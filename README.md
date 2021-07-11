# Hello-rust project

## Description.

Simple implementation for exercise: https://github.com/massbitprotocol/helloworld-rust-project

## Instruction

1. git clone https://github.com/vuviettai/hello-rust.git
2. cd hello-rust
3. open 5 terminals and run ``` cargo run {port} ``` from each terminal (if {port} is omitted some random port is used)
4. Open other terminal for client end try put/get command via curl
For example:
   ```
   curl --header 'Content-Type: application/json' -d '{"jsonrpc": "2.0", "method": "put", "params": [{ "name": "pokemon1", "color": "blue", "eye_num": 1, "nose_num": 2, "mouth_num": 3}], "id": 1}' -o - http://127.0.0.1:{port}
   curl --header 'Content-Type: application/json' -d '{"jsonrpc": "2.0", "method": "put_value", "params": ["pokemon2", "blue", 2, 3, 4], "id": 1}' -o - http://127.0.0.1:{port}
   curl --header 'Content-Type: application/json' -d '{"jsonrpc": "2.0", "method": "get", "params": ["pokemon1"], "id": 1}' -o - http://127.0.0.1:{port}
   ```