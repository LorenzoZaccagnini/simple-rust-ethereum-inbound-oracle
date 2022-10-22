use dotenv::dotenv;
use web3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let alchemy_api_key = dotenv::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set");
    let web3 = web3::Web3::new(web3::transports::WebSocket::new(&alchemy_api_key).await?);

    let contract_address = "0x06012c8cf97BEaD5deAe237070F9587f8E7A266d";
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
        .for_each(|log| async {
            println!("log: {:?}", log);
        })
        .await;

    Ok(())
}
