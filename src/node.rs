use ethers::core::utils::hex;
use ethers::prelude::*;
use eyre::Result;
use k256::SecretKey;
use std::{convert::TryFrom, sync::Arc, time::Duration};

pub struct Node {
    pub client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    // Anvil specific feature
    pub http_endpoint: String,
    pub ws_endpoint: String,
}

impl Node {
    /// Reads from env variables `ETH_RPC_URL` and `MERGEBOT_PRIVKEY and `ETH_RPC_URL`
    pub async fn new_local_node_from_env() -> Result<Self> {
        let priv_key: String = std::env::var("MERGEBOT_PRIVKEY")?;
        let priv_key = hex::decode(priv_key)?;
        let http_endpoint = std::env::var("ETH_RPC_URL")?;
        let ws_endpoint = std::env::var("ETH_WSS_URL")?;
        println!("http_endpoint: {:?}", http_endpoint);
        println!("ws_endpoint: {:?}", ws_endpoint);

        let provider = Provider::<Http>::try_from(http_endpoint.clone())?
            .interval(Duration::from_millis(10u64));

        let wallet: LocalWallet = SecretKey::from_be_bytes(&priv_key)
            .expect("did not get private key")
            .into();
        println!("Wallet with address: {:?}", wallet.address().clone());
        let provider = provider.with_sender(wallet.address());
        let chain_id: u64 = provider.get_chainid().await?.as_u64();
        let wallet = wallet.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        Ok(Node {
            client,
            http_endpoint,
            ws_endpoint,
        })
    }
}
