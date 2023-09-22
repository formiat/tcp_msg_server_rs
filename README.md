# Rust TCP Message Server

## Overview

This Rust-based server application listens on a given TCP port and performs the following tasks:

1. Accepts incoming TCP connections.
2. Receives a message from the client (maximum 128 bytes).
3. Writes the received message to a text file named `<port_num>.txt`.
4. Waits for 3 seconds.
5. Sends an "ACCEPTED" message back to the client.

The server handles multiple clients concurrently.

## Requirements

- Rust toolchain
- Cargo package manager

## Installation

1. Clone the repository to your local machine.

    ```bash
    git clone https://github.com/formiat/tcp_msg_server_rs.git
    ```

2. Navigate to the repository folder.

    ```bash
    cd tcp_msg_server_rs
    ```

3. Build the project using Cargo.

    ```bash
    cargo build --release
    ```

## Usage

1. Run the server by specifying the port number as an argument.

    ```bash
    cargo run --release 12345
    ```

   This will start the server on port 12345, and a text file named `12345.txt` will be created for storing incoming messages.

2. Connect to the server using a TCP client, such as telnet. Open a new terminal window and execute:

    ```bash
    telnet 127.0.0.1 12345
    ```

3. Send a message (up to 128 bytes).

4. You will receive an "ACCEPTED" message from the server after 3 seconds.

5. The sent message will be appended to the `12345.txt` file.

6. To disconnect from the server, press `Ctrl+]` followed by typing `quit` and pressing Enter (in the telnet session).

## Notes

- The server is designed to handle multiple clients concurrently. Hence, if two clients send messages at the same time, both will receive the "ACCEPTED" message approximately 3 seconds later, not 6.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
