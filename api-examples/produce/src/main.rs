use futures::stream::StreamExt;
use std::process::exit;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::ReplicaLeader;
use flv_client::SpuController;
use flv_future_aio::io::stdin;
use flv_future_aio::io::AsyncBufReadExt;
use flv_future_aio::io::BufReader;
use flv_future_aio::task::run_block_on;

fn main() {
    run_block_on(producer()).expect("run");
}

async fn producer() -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // look-up stream for "my-topic-1"
    let topic = "my-topic-1";
    let partition = 0;
    let mut replica = client
        .find_replica_for_topic_partition(topic, partition)
        .await
        .expect("replica missing");

    // read from terminal and send to producer
    let stdin = stdin();
    let std_lock = stdin.lock().await;
    let mut lines = BufReader::new(std_lock).lines();
    while let Some(line) = lines.next().await {
        let text = line?;
        let record = text.as_bytes().to_vec();
        if let Err(err) = replica.send_record(record).await {
            eprintln!("{}", err);
            exit(-1);
        }
    }

    Ok(())
}
