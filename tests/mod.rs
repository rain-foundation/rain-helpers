use rain_helpers::{fetch_rain_borrowers, fetch_rain_pools};
use solana_program::pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey};

const RPC_URL: &str = "";
const INF_TOKEN: Pubkey = pubkey!("5oVNBeEEQvYi1cX3ir8Dx5n1P7pdxydbGF2X4TxVusJm");

#[tokio::test]
async fn main() {
    let client =
        RpcClient::new_with_commitment(String::from(RPC_URL), CommitmentConfig::confirmed());

    let suppliers = fetch_rain_pools(&client, &INF_TOKEN)
        .await
        .expect("Handle result");

    println!("{:#?}", suppliers);

    let borrowers = fetch_rain_borrowers(&client, &INF_TOKEN)
        .await
        .expect("Handle result");
    println!("{:#?}", borrowers);
}
