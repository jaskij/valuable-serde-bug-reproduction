This repository contains a reproduction of what seems to be a bug in `valuable-serde`.
There's two versions of a struct, wrapping `Valuable` around `anyhow::Error`.


### working

This version stringifies the values and holds them in a struct which derives `Valuable`.

```shell
$ cd working
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/jaskij/projects/rust/valuable_serde_repro/target/debug/working`
JSON: {"message":"asdf FOO","root_cause":"\"asdf FOO\"","chain":[]}
```

### not_working

This version `anyhow::Error` and manually implements `Valuable`.

```shell
$ cd not_working
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `/home/jaskij/projects/rust/valuable_serde_repro/target/debug/not_working`
FAILED: Error("asdf FOO", line: 0, column: 0)
```
