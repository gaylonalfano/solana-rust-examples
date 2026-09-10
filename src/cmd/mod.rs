use clap::Args;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signer::keypair::Keypair;
use std::sync::Arc;

pub mod associated_token_balance;
pub mod create_spl;
pub mod creation_date;
pub mod fetch_idl;
pub mod list_nfts;
pub mod mint_spl;
pub mod new_wallet;
pub mod nft_owner;
pub mod pubsub;
pub mod transfer_sol;
pub mod wrap_sol;

#[derive(Args)]
pub struct RpcArgs {
    /// Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
    #[arg(long, env = "RPC_URL")]
    rpc_url: url::Url,
}

impl RpcArgs {
    pub fn client(&self) -> RpcClient {
        RpcClient::new(self.rpc_url.to_string())
    }
}

#[derive(Args)]
pub struct SignerArgs {
    /// Base58 encoded keypair that signs and pays for the transaction
    #[arg(
        long = "signer-keypair",
        env = "SIGNER_KEYPAIR",
        hide_env_values = true,
        value_parser = parse_keypair
    )]
    pub keypair: Arc<Keypair>,
}

/// `Keypair` isn't `Clone`, which clap requires of parsed values, so it's wrapped in an `Arc`.
pub fn parse_keypair(s: &str) -> Result<Arc<Keypair>, String> {
    Keypair::try_from_base58_string(s)
        .map(Arc::new)
        .map_err(|e| e.to_string())
}
