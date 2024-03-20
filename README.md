# Berkeley Slave Node

The `berkeley_slave` program is a Rust-based implementation of the slave node for the Berkeley Clock Synchronization Algorithm. It's designed to adjust its system time based on synchronization instructions received from a master node.

## Features

- Listens for time synchronization commands from the master node.
- Adjusts its internal clock based on the received instructions.
- Uses UDP for network communication.

## Requirements

To compile and run this program, you will need:

- Rust programming environment (see [the Rust installation guide](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

## Compilation

Navigate to the root directory of the `berkeley_slave` project and run the following command to compile the program:

```bash
cargo build --release
```

This command generates an executable in the `target/release` directory.

## Running the Slave Node

To run the slave node, use the following command syntax:

```bash
cargo run -- [Slave Node Address]
```

- `[Slave Node Address]` is the IP address and port that the slave node binds to and listens on (e.g., `127.0.0.1:8081`).

### Example

```bash
cargo run -- 127.0.0.1:8081
```

This command starts the slave node on `127.0.0.1:8081`, making it ready to listen for time synchronization messages from the master node.
