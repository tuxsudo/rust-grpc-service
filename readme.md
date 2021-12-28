# Sample Rust grpc server

## Getting started

In root of project, run:

```shell
cargo build;
cargo run;
```

The grpc server should be running on port `50051`.

## Testing endpoints

1. Install grpc client [bloomrpc](https://github.com/bloomrpc/bloomrpc).
2. Import proto at from `src/build_cost/build_cost.proto`.
3. Change host to `0.0.0.0:50051`
4. Select grpc method to run, change request arguments, repeat.
