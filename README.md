# Tonic gRPC Example

Simple example of gRPC implementation with [tonic](https://docs.rs/tonic/latest/tonic/) and async work with [mongodb](https://docs.rs/mongodb/latest/mongodb/) and example of use [env_logger](https://docs.rs/env_logger/latest/env_logger/).

## Setup Logging

```bash
$env:RUST_LOG='error,tonic_grpc_example=debug'; cargo r
```

## Setup MongoDB

```bash
docker run --name tonic-grpc-mongo -p 27017:27017 -d mongo:latest
```
