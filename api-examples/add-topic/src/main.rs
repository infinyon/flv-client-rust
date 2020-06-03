use futures::stream::StreamExt;

use flv_client::profile::ScConfig;
use flv_client::ClientError;
use flv_client::query_params::ReplicaConfig;
use flv_client::FetchLogOption;
use flv_client::FetchOffset;
use flv_client::ReplicaLeader;
use flv_client::SpuController;
use flv_future_aio::task::run_block_on;

fn main() {
    run_block_on(add_topic()).expect("run");
}

async fn add_topic() -> Result<(), ClientError> {
    let config = ScConfig::new(None, None).expect("connect");
    let mut client = config.connect().await.expect("should connect");

    // look-up stream for "my-topic-1"
    let topic = "my-topic-1";
    let partition = 0;
    let res = client.create_topic("test1".to_owned(), ReplicaConfig::Computed(1,2,true), false).await;
//    let res = client.get_topic_composition(&topic).await;

    println!("{:?}", res);

    Ok(())
}
