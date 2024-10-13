# RUDP Protocol Service (Rust)
Rust Reliable UDP Network Protocol (Protocol Structures, Protocol API) Client-Server App w/ ScyllaDB Payload Storage



![rudp-protocol-svc](docs/rudp-protocol-arch.png)


## Creating the Project Structure

```shell
cargo new --vcs none rudp-protocol-svc
```

Next.

```shell
cd rudp-protocol-svc
```

Next add the following `workspace` configuration to the top of the root `Cargo.toml` file.

```shell
workspace = { members = [
    "rudp-protocol-kit",
    "rudp-socket-client",
    "rudp-socket-server",
    "storage-kit",
] }
```

Next.

Create the `rudp-protocol-kit` library crate folder
```shell
cargo new --lib rudp-protocol-kit`
```

This results in the following layout.

```shell
rudp-protocol-kit/
├── src
│   └── lib.rs
└── Cargo.toml
```

Next.

Create the `storage-kit` library crate folder
```shell
cargo new --lib storage-kit
```

This results in the following layout.

```shell
storage-kit/
├── src
│   └── lib.rs
└── Cargo.toml
```

Next create the RUDP socket client and RUDP socket server binary crates.

First the RUDP socket client.

```shell
cargo new --bin rudp-socket-client
```

This results in the following layout.






## Compile Entire Workspace

```shell
cargo build
```

## Running the RUDP Client and RUDP Server

```shell
cargo run -p rudp-socket-client
cargo run -p rudp-socket-server
```

## Running Tests in the Crate Dependencies RUDP-Protocol-Kit and Storage-Kit

```shell
cargo test -p rudp-protocol-kit
cargo test -p storage-kit
```


## Project Structure

The following layout is final project structure.

```shell
rudp-protocol-svc
├── Cargo.toml
├── Dockerfile.client
├── Dockerfile.server
├── Makefile
├── docker-compose.yaml
├── rudp-protocol-kit
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       └── rudp.rs
├── rudp-socket-client
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── rudp-socket-server
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── storage-kit
    ├── Cargo.toml
    └── src
        ├── lib.rs
        ├── storage-repository.rs
        └── storage.rs
```
