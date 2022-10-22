use dotenv::dotenv;
use ethnum::U256;
use web3;
use web3::futures::{future, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let alchemy_api_key = dotenv::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set");
    let web3 = web3::Web3::new(web3::transports::WebSocket::new(&alchemy_api_key).await?);

    let contract_address = "0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85";
    let event_signature = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

    let filter = web3::types::FilterBuilder::default()
        .address(vec![contract_address.parse().unwrap()])
        .from_block(web3::types::BlockNumber::Latest)
        .topics(
            Some(vec![event_signature.parse().unwrap()]),
            None,
            None,
            None,
        )
        .build();

    let transfer_listen = web3.eth_subscribe().subscribe_logs(filter).await?;

    transfer_listen
        .for_each(|log| {
            let id = format!("{:?}", log.unwrap().topics[3]);
            println!("id NOT decoded: {:?}", id);
            let id_decoded = U256::from_str_radix(&id[2..], 16).unwrap();
            println!("id decoded: {:?}", id_decoded);
            println!("----------");
            future::ready(())
        })
        .await;

    Ok(())
}
