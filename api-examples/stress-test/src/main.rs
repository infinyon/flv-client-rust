use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::exit;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::ReplicaLeader;
use flv_client::SpuController;
use flv_future_aio::task::run_block_on;

async fn stress_test() -> Result<(), ClientError> {
    let (file_name, loop_cnt) = get_params();
    let lines = lines_from_file(file_name.as_str());

    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // look-up stream for "stress-test-topic"
    let topic = "stress-test-topic";
    let partition = 0;
    let mut replica = client
        .find_replica_for_topic_partition(topic, partition)
        .await
        .expect("replica missing");

    for _ in 0..loop_cnt {
        for line in &lines {
            let record = line.as_bytes().to_vec();
            let byte_cnt = record.len();

            match replica.send_record(record).await {
                Ok(()) => println!("sent {} bytes", byte_cnt),
                Err(err) => {
                    eprintln!("{}", err);
                    exit(-1);
                }
            }
        }
    }

    Ok(())
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_params() -> (String, u32) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: stress-test <file-name> <loop-count>");
        exit(-1);
    }
    (
        (args[1]).to_string(),
        (args[2].to_string().parse::<u32>().unwrap()),
    )
}

fn main() {
    run_block_on(stress_test()).expect("run");
}
