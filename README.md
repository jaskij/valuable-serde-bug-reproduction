This repository contains a reproduction of what seems to be a bug in `valuable-serde`.
There's two versions of a struct, wrapping `Valuable` around `anyhow::Error`.

### Working version

This version stringifies the values and holds them in a struct which derives `Valuable`.
The struct `ValuableAnyhow` is properly serialized and can be printed.

Additionally in this directory, there is an `expanded.rs` which contains results of `cargo expand`.

```shell
$ cd working
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/jaskij/projects/rust/valuable_serde_repro/target/debug/working`
JSON: {"message":"asdf FOO","root_cause":"\"asdf FOO\"","chain":[]}
```

### Not working

This version `anyhow::Error` and manually implements `Valuable`.
Here, the struct fails to serialize properly and we get a very unhelpful error from `serde_json`.

```shell
$ cd not_working
$ cargo run --bin not_working
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/jaskij/projects/rust/valuable_serde_repro/target/debug/not_working`
FAILED: Error("asdf FOO", line: 0, column: 0)
```

### Tracing

Additionally, `tracing-subscriber` seems to silently swallow events which fail to serialize.

```shell
$ cd not_working
$ cargo run --bin tracing
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/jaskij/projects/rust/valuable_serde_repro/target/debug/tracing`
```