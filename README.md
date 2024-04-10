# Berkeley Slave Node

The `berkeley_slave` program is a Rust-based implementation of the slave node for the Berkeley Clock Synchronization
Algorithm. It's designed to adjust its system time based on synchronization instructions received from a master node.

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

- `[Slave Node Address]` is the IP address and port that the slave node binds to and listens on (
  e.g., `127.0.0.1:8081`).

### Example

```bash
cargo run -- 127.0.0.1:8081
```

This command starts the slave node on `127.0.0.1:8081`, making it ready to listen for time synchronization messages from
the master node.

### Docker Support

The `berkeley_slave` application can also be run as a Docker container, which simplifies deployment and ensures
consistent runtime environments.

#### Building the Docker Image

First, ensure Docker is installed on your system. Then, build the Docker image for berkeley_slave by navigating to the
root directory of the project and running:

```bash
docker build -t berkeley_slave .
```

This command builds a Docker image named berkeley_slave based on the Dockerfile in the current directory.

#### Running the Docker Container

After building the image, run the slave node within a Docker container using:

```bash
docker run --net=host -e ARGS="[Slave Node Address]" berkeley_slave
```

Replace `[Slave Node Address]` with the IP address and
port combination the slave node should bind to and listen on.

#### Example

If your slave node application listens on `127.0.0.1:8081` and you want to use port `8081` on your host:

```bash
docker run --net=host -e ARGS="127.0.0.1:8081" berkeley_slave
```
