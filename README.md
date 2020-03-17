# Rust client API examples for Fluvio Streaming Platform

## Build

To build

```
cargo build
```


## Start Producer

Start producer command requires a topic and a partition.  Server address defaults to '0.0.0.0:9003'.

```
./target/debug/flv-produce-example
```

## Start Consumer

Start consumer command requires a topic and a partition. 

```
./target/debug/flv-consume-example
```