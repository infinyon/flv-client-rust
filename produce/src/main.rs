
use std::process::exit;

use futures::stream::StreamExt;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::SpuController;
use flv_client::ReplicaLeader;
use flv_future_aio::io::stdin;
use flv_future_aio::io::BufReader;
use flv_future_aio::io::AsyncBufReadExt;
use flv_future_aio::task::run_block_on;

fn main() {

    run_block_on(print_stream()).expect("run");
    
}

async fn print_stream() -> Result<(),ClientError> {

     
    println!("connecting to sc at localhost");

    let config = ScConfig::new(Some("localhost:9003".to_owned()), None)?;

    // connect to sc
    let mut client = config.connect().await?;

    // find replica
    let mut replica = client.find_replica_for_topic_partition("test1",0).await?;

    println!("found topic test1, replication 0");

    let stdin = stdin();

    let std_lock = stdin.lock().await;
    let mut lines = BufReader::new(std_lock).lines();
    while let Some(line) = lines.next().await {
        let text = line?;
        let record = text.as_bytes().to_vec();
        if let Err(err) =  replica.send_record(record).await {
            eprintln!("{}", err);
            exit(-1);
        }
    }

    Ok(())

}