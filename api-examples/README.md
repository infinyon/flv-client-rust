# Rust Client Sample API - for Fluvio Streaming Platform

## Requirement

These code samples assume you have access to a Fluvio cluster. To checkout your cluster profile, run the following command:

```
> ./target/debug/fluvio profile view
```

By default, the topics are created with 1 partition and 1 replication factor.


## Build Binaries

Build all binaries by running the following command at the top of the tree:

```
> cargo build
```

The build script generates the following binaries:

```
./target/debug/topic-ops
./target/debug/produce
./target/debug/consume
./target/debug/fetch-last
./target/debug/stress-test
```


## Topic Operations

Topic operations has sample APIs to create, get, list, and delete topics:

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


## Produce

The producer assumes that 'my-topic-1' topic has been created.
To create a topic, you may use the CLI or run the script above:

```
> ./target/debug/topic-ops create my-topic-1
```


### Start Producer

The producers waits for user input and feeds the topic one message per line.
Start the producer and generate a few messages:

```
> ./target/debug/produce
this is line 1
this is line 2
```


## Consume

The producer reads from 'my-topic-1' and it assumes it has been created.

### Start Consumer

Consumer reads all messages currently in the topic and continues listening for additional messages. 

Start the producer and read messages:

```
> ./target/debug/flv-consumer
this is line 1
this is line 2
```

## Fetch Last

Fluvio can fetch records from different offsets in the stream:

* an exact offset,
* a relative offset from the beginning
* a relative offset from  the end

This example show how to read the last records in the stream. 

The API reads from 'my-topic-1' and it assumes it has been created.

To fetch last message:

```
> ./target/debug/fetch-last
this is line 2
```


## Stress Test

The producer assumes that 'stress-test-topic' topic has been created. To create the topic, run the following command:

```
> ./target/debug/topic-ops create stress-test-topic
```

The stress test API reads records from a file and sends each line to 'stress-test-topic'. The script also has a loop parameter that loops through the records as many times as desired.

There are three data files provided in the 'test-data' directory:

* one-record-small.txt
* one-record.txt
* records.txt (30 records)

A loop counter of 2 used in combination with 'records.txt' sends 30 records 2 times to a total of 60 records.

#### Run Stress Test

```
> ./target/debug/stress-test ./stress-test/test-data/records.txt 2
```

