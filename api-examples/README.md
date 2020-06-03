# Rust Client Sample API - for Fluvio Streaming Platform

## Topic Operations



## Producer/Consumer

Producer/Consumer tests assume that a topic 'my-topic-1' has been created.
To create a topic through fluvio cli:

```
> fluvio topic create my-topic-1
```

### Build Binaries

Build producer/consumer by running the following command at the top of the tree:

```
> cargo build
```

The build script generates the following binaries:
```
./target/debug/flv-consumer
./target/debug/flv-producer
```

#### Start Producer

The producers waits for user input and feeds the topic one message per line.

Open terminal, start producer, and generate a few messages:

```
> ./target/debug/flv-producer
this is line 1
this is line 2
```

#### Start Consumer

Consumer reads all messages stored in the topic and continues listening for additional messages. 

Open a new terminal and start consumer:

```
> ./target/debug/flv-consumer
this is line 1
this is line 2
```
