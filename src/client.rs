pub mod myclient {

    use {
        crate::{model::ServerPrivate, runtime},
        solana_account_decoder::UiAccountEncoding,
        solana_client::pubsub_client::PubsubClient,
        solana_program::pubkey::Pubkey,
        solana_rpc_client_api::config::RpcAccountInfoConfig,
        solana_sdk::commitment_config::CommitmentConfig,
        solana_sdk::signer::keypair::Keypair,
        std::io::Result,
    };

    pub struct ServerInfo {
        pub payer: Pubkey,
    }

    const PROGRAM_ADDRESS: &str = "3LmEiESDDKbEKJBXrYnYiS4CX5dbcj6xnWitoHRW2bwQ";
    const SEED: &str = "solana-server";
    const PRIVATE_PATH: &str = "./private/private.json";

    pub fn init_solana_wallet() -> std::io::Result<Keypair> {
        // open config file
        let config_path = runtime::app_path(PRIVATE_PATH);
        match std::fs::read_to_string(config_path) {
            Ok(config) => {
                let config = serde_json::from_str::<ServerPrivate>(&config).unwrap();
                Ok(Keypair::from_base58_string(&config.secret))
            }
            Err(e) => {
                println!("error reading config file : {}", e);
                let wallet = Keypair::new();
                let s = &ServerPrivate {
                    secret: wallet.to_base58_string(),
                };
                let secret_config = serde_json::to_string(s).unwrap();
                std::fs::write(PRIVATE_PATH, secret_config)?;
                Ok(wallet)
            }
        }
    }

    pub fn get_server_info() -> ServerInfo {
        return ServerInfo {
            payer: Pubkey::new_unique(),
        };
    }

    pub fn get_account_updates(account_pubkey: &Pubkey) -> Result<()> {
        let (mut _account_subscription_client, account_subscription_receiver) =
            PubsubClient::account_subscribe(
                runtime::WS_URL,
                account_pubkey,
                Some(RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::JsonParsed),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    min_context_slot: None,
                }),
            )
            .unwrap();

        loop {
            match account_subscription_receiver.recv() {
                Ok(response) => {
                    println!("account subscription response: {:?}", response);
                }
                Err(e) => {
                    println!("account subscription error: {:?}", e);
                    break;
                }
            }
        }
        Ok(())
    }
}
