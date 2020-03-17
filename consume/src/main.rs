
use futures::stream::StreamExt;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::SpuController;
use flv_client::FetchLogOption;
use flv_client::FetchOffset;
use flv_client::ReplicaLeader;
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


    let mut stream = replica.fetch_logs(FetchOffset::Earliest, FetchLogOption::default());

    println!("found topic test1, replication 0");

    while let Some(partition_response) = stream.next().await {
        let records = partition_response.records;

        for batch in records.batches {
            for record in batch.records {
                if let Some(bytes) = record.value().inner_value() {
                    let msg = String::from_utf8(bytes).expect("string");
                    println!("{}", msg);
                }
            }
        }
    }
    

    Ok(())

}