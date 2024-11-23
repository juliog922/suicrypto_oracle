# SuiCrypto Oracle

A WebSocket server that manages clients based on tokens listed in `config.json`. Every 10 seconds, it requests token price, symbol, and datetime from each client. The server fetches this data using the CoinGecko API for tokens on the Sui blockchain via their `contract_address`.

## Prerequisites

Before running the project, you will need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (via `rustup` and `cargo`)

## Setup Instructions

1. **Clone the repository**

   Clone the repository to your local machine:

   ```bash
   git clone https://github.com/juliog922/suicrypto_oracle.git
   cd suicrypto_oracle

2. **Install Rust**

    Make sure you have `rustup` and `cargo` installed. Follow the installation instructions [here](https://www.rust-lang.org/tools/install).

3. **Configure the `.env` filet**

    The project uses a `.env` file for configuration. Create a `.env` file in the root directory with the following:

    ```bash

    SERVER_HOST=127.0.0.1:8080
    RUST_LOG=info

- `SERVER_HOST` specifies the address where the WebSocket server will listen. If not set, it defaults to `127.0.0.1:8080`
- `RUST_LOG` controls the log level (e.g., info, warn). Set it to info to see detailed logs.

4. **Configure the `tokens.json` file**

    Create or modify the `tokens.json` file to include a list of tokens:

    ```json
    {
    "tokens": ["DEEP", "SUI", "SUDENG"]
    }

- The tokens key should contain a list of token names.
- If the file doesn't contain this structure, the program will throw an error.
- If the tokens are misspelled or not found on the Sui network, a warning will appear.

5. **Runing the Server**

    To start the WebSocket server, use the following command:

    ```bash
    cargo run --bin server

- Once the server starts, you will see the following message in the terminal:

    ```bash
    Server listening on 127.0.0.1:8080

- If there are no clients connected, the server will print the following warning:

    ```bash
    Broadcast Channel Error: No clients listening

6. **Running the Clients**

    In a separate terminal, start the client for each token by running:

    ```bash
    cargo run --bin client

- Once the client connects, you will see:

    ```bash
    Client created: <token_name>
    Client connected with Token: <token_name>

- Once the server requests token data, the client will send the price, symbol, and datetime in this format:

    ```bash
    Message received from client: {"price":<token_price>,"symbol":<token_symbol>,"timestamp":<token_price_datetime>}

## Log Levels

The logs are printed using the `log` crate, with `info` and warn levels.
If `RUST_LOG=info` is set, you will see detailed log messages. Otherwise, only warnings will appear.

## Disconnecting

- When the server shuts down or a client disconnects, you will see:

    ```bash
    Client disconnected

## Running Tests

- To run the project's tests, execute the following command:

    ```bash
    cargo test

This will run all unit and integration tests in the project.

## Notes

- Ensure the token names in `tokens.json` are correctly spelled and belong to the Sui network.

- The server and client can be run on separate machines as long as they can connect to each other via the configured `SERVER_HOST`.