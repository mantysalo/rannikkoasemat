# Rannikkoasemat

> Weather stations for coastal areas

Live at https://mantysalo.fi/rannikkoasemat/

Rannikkoasemat is a Rust-based web application that provides real-time weather data for various coastal locations in Finland. It simplifies the process of accessing weather information, making life easier for sailors, fishermen, and anyone interested in coastal weather conditions.

The application has been designed to be extremely lightweight, so that weather information can be accessed even with poor internet connectivity, which is often the case at sea.

## Getting started

Rannikkoasemat is built with Rust so to get started, ensure you have Rust and Cargo installed. See https://www.rust-lang.org/learn/get-started for more information.

```shell
git clone https://github.com/mantysalo/rannikkoasemat.git
cd rannikkoasemat
cargo build
cargo run
```

This starts a local server. Access the web application by navigating to http://localhost:8000 in your web browser.

## Running the application

There are two ways of running the application: using the binary, or running it inside a container.

### Binary

To build a release binary, run the following command

```shell
cargo build --release
```

This creates an executable in `target/release` which can be run, and the application can then be accessed at localhost:8000

### Docker / Container

To build and run the Dockerfile, run the following commands:

```shell
docker build -t rannikkoasemat:latest .
docker run -p 8000:8000 rannikkoasemat:latest
```

This starts a docker container with the application running which you can then access at localhost:8000
