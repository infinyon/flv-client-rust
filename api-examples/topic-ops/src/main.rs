use std::env;
use std::process::exit;

use flv_client::profile::ScConfig;
use flv_client::query_params::ReplicaConfig;
use flv_client::ClientError;
use flv_client::SpuController;
use flv_future_aio::task::run_block_on;

async fn create_topic(topic: String) -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // topic parameters
    let (partitions, replicas) = (1, 2);
    let ignore_rack_assign = true;
    let validate_only = false;

    // create topic
    let res = client
        .create_topic(
            topic,
            ReplicaConfig::Computed(partitions, replicas, ignore_rack_assign),
            validate_only,
        )
        .await
        .expect("should create topic");

    println!("{:?}", res);

    Ok(())
}

async fn list_topic() -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");
    let res = client
        .topic_metadata(None)
        .await
        .expect("should list topics");

    println!("{:?}", res);
    Ok(())
}

async fn get_topic(topic: String) -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");
    let res = client
        .topic_metadata(Some(vec![topic]))
        .await
        .expect("should list topics");

    if res.len() > 0 && res[0].error.is_some() {
        println!("Error {:?}", res[0].error.unwrap());
    } else {
        println!("{:?}", res);
    }

    Ok(())
}

async fn delete_topic(topic: String) -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // delete topic
    let res = client
        .delete_topic(topic.as_str())
        .await
        .expect("should create topic");

    println!("{:?}", res);

    Ok(())
}

fn main() {
    let (command, topic) = get_params();
    match command.as_str() {
        "create" => run_block_on(create_topic(topic)).expect("run"),
        "get" => run_block_on(get_topic(topic)).expect("run"),
        "list" => run_block_on(list_topic()).expect("run"),
        "delete" => run_block_on(delete_topic(topic)).expect("run"),
        &_ => {}
    }
}

fn get_params() -> (String, String) {
    let mut err = false;
    let mut command = String::from("");
    let mut topic = String::from("");

    // Lookup args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        err = true;
    } else {
        if !vec!["create", "get", "list", "delete"].contains(&args[1].as_str()) {
            err = true;
        } else {
            command = (&args[1]).to_string();
            if command != String::from("list") {
                if args.len() == 3 {
                    topic = (&args[2]).to_string();
                } else {
                    err = true;
                }
            }
        }
    }

    if err {
        println!("Usage: topic-ops [create|get|list|delete] <topic-name>");
        exit(-1);
    }

    (command, topic)
}
