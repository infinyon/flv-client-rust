use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::FetchLogOption;
use flv_client::FetchOffset;
use flv_client::ReplicaLeader;
use flv_client::SpuController;
use flv_future_aio::task::run_block_on;

fn main() {
    run_block_on(fetch_last()).expect("run");
}

async fn fetch_last() -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    let topic = "my-topic-1";
    let partition = 0;
    let last_offset = FetchOffset::Latest(Some(1));

    let mut replica = client
        .find_replica_for_topic_partition(topic, partition)
        .await
        .expect("find replica");
    let response = replica
        .fetch_logs_once(last_offset, FetchLogOption::default())
        .await
        .expect("fetch long once");

    // read from producer and print to terminal
    let records = response.records;
    for batch in records.batches {
        for record in batch.records {
            if let Some(bytes) = record.value().inner_value() {
                let msg = String::from_utf8(bytes).expect("string");
                println!("{}", msg);
            }
        }
    }

    Ok(())
}

/*
async fn fetch_logs_once(
    &mut self,
    offset_option: FetchOffset,
    option: FetchLogOption
) -> Result<FetchablePartitionResponse<DefaultRecords>,ClientError>  {
*/
