
# Grpc Cartesi Machine Client

## Introduction

This crate is implementation of grpc Cartesi machine client for Cartesi machine server. 


### Requirements

- Rust >= 1.51
- [Machine Emulator](https://github.com/cartesi/machine-emulator)

## Native build for Linux and MacOS

### Clone repository
```console
$ git clone --recurse-submodules git@github.com:cartesi/machine-manager.git
```

The Cartesi machine emulator server has pre-built Docker images published at [Docker Hub](https://hub.docker.com/repository/docker/cartesi/machine-emulator).

### Build grcp cartesi machine library and test client
```console
$ cd grpc-cartesi-machine
$ cargo build
```

### Run tests

Local  cartesi machine server binary, emulator libraries and images must be placed in folder `/opt/cartesi`

```console
$ sudo mkdir /opt/cartesi
$ docker pull cartesi/machine-emulator:0.7.0
$ docker run -dit cartesi/machine-emulator:latest /bin/bash
$ sudo docker cp <docker container running name acquired with docker ps>:/opc/cartesi /opt/
$ sudo tests/download-test-images.sh
```
Alternatively build and install cartesi machine server from [machine emulator](https://github.com/cartesi/machine-emulator) repository.

Start the grpc cartesi machine tests:

```console
$ cargo test -- --test-threads 1
```

## Executing the test client

Start Cartesi machine server in local terminal:

```console
$ /opt/cartesi/bin/remote-cartesi-machine --server-address=127.0.0.1:50051
```
Once you have the server up and running, you can run example test client

```console
$ cargo run --bin grpc-cartesi-machine-test http://127.0.0.1:50051
```

## Contributing

Thank you for your interest in Cartesi! Head over to our [Contributing Guidelines](CONTRIBUTING.md) for instructions on how to sign our Contributors Agreement and get started with Cartesi!

Please note we have a [Code of Conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Authors

- *Marko Atanasievski*

## License

The machine-manager repository and all contributions are licensed under
[APACHE 2.0](https://www.apache.org/licenses/LICENSE-2.0). Please review our [LICENSE](LICENSE) file.

## Acknowledgments

- Original work
