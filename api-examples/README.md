# Rust Client Sample API - for Fluvio Streaming Platform

### Build Binaries

Build all binaries by running the following command at the top of the tree:

```
> cargo build
```

The build script generates the following binaries:
```
./target/debug/flv-consumer
./target/debug/flv-producer
./target/debug/topic-ops
```


## Topic Operations

Topic operations has sample APIs for topic create, list and delete:

### Topic Create

```
> ./target/debug/topic-ops create test1
test1
```

### Get Topic

```
> ./target/debug/topic-ops get test1
[TopicMetadata { name: "test1", error: None, topic: Some(Topic { type_computed: true, assigned_partitions: None, partitions: Some(1), replication_factor: Some(2), ignore_rack_assignment: true, status: Provisioned, reason: "", partition_map: Some([PartitionReplica { id: 0, leader: 5003, replicas: [5003, 5001], live_replicas: [0, 5001] }]) }) }]
```


### List Topics

```
> ../target/debug/topic-ops list
[TopicMetadata { name: "test1", error: None, topic: Some(Topic { type_computed: true, assigned_partitions: None, partitions: Some(1), replication_factor: Some(2), ignore_rack_assignment: true, status: Provisioned, reason: "", partition_map: Some([PartitionReplica { id: 0, leader: 5001, replicas: [5001, 5002], live_replicas: [0, 5002] }]) }) }]
```

### Topic Delete

```
> ./target/debug/topic-ops create test1
test1
```


## Producer/Consumer

Producer/Consumer requires the 'my-topic-1' topic to be created.
The topic may be created through Fluvio ClI or the script above:

```
> ./target/debug/topic-ops create my-topic-1
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

#### Stress Test

```
> ./target/debug/topic-ops create stress-test-topic
> ./target/debug/stress-test ./stress-test/test-data/records.txt 10
```