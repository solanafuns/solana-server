use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signer},
    solana_server::caddy::{self},
    solana_server::{client::myclient, runtime},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    runtime::confirm_dir();
    let s = caddy::new_server_config();
    let body = serde_json::to_string_pretty(&s).unwrap();
    println!("caddy config : {}", body);

    let wallet = myclient::init_solana_wallet().unwrap();
    let wallet_pub = &wallet.pubkey();
    // let info = myclient::get_server_info();
    // println!("payer address : {}", info.payer);
    let connection =
        RpcClient::new_with_commitment(runtime::RPC_URL.to_string(), CommitmentConfig::finalized());
    let balance = connection.get_balance(&wallet.pubkey()).unwrap();
    println!("balance is : {}", balance);

    if balance <= runtime::LAMPORTS_PER_SOL {
        let transaction = connection
            .request_airdrop(wallet_pub, runtime::LAMPORTS_PER_SOL * 3)
            .unwrap();
        println!("airdrop transaction : {:?}", transaction);
        connection
            .confirm_transaction_with_commitment(&transaction, CommitmentConfig::finalized())
            .unwrap();
    }
    let balance = connection.get_balance(&wallet_pub).unwrap();
    println!("balance is : {}", balance);

    println!("begin watch wallet : {}", wallet_pub);

    myclient::get_account_updates(wallet_pub).unwrap();

    Ok(())
}
