use anyhow::{Context, bail};
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey};

use super::RpcArgs;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    /// The pubkey address of the SPL Token mint account
    #[arg(long, env = "MINT_ACCOUNT_PUBKEY")]
    mint_account_pubkey: Pubkey,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = self.rpc.client();
        let mint = self.mint_account_pubkey;

        let account = fetch_nft_account(&client, &mint).await?;

        let data = account
            .data
            .decode()
            .context("Failed to decode account data")?;
        let token_account = spl_token_interface::state::Account::unpack(&data)?;

        println!("{} owner:\n{}", mint.to_string(), token_account.owner);

        Ok(())
    }
}

async fn fetch_nft_account(
    client: &RpcClient,
    mint: &Pubkey,
) -> anyhow::Result<solana_client::rpc_response::UiAccount> {
    let filters = Some(vec![
        // account size
        RpcFilterType::DataSize(165),
        // mint bytes
        RpcFilterType::Memcmp(Memcmp::new(0, MemcmpEncodedBytes::Base58(mint.to_string()))),
        // amount bytes = 1
        RpcFilterType::Memcmp(Memcmp::new(
            64,
            // Bytes filter seems to be failing:
            //MemcmpEncodedBytes::Bytes(
            //1u64.to_le_bytes().to_vec(), // Little-endian encoding of 1
            //),
            MemcmpEncodedBytes::Base64("AQAAAAAAAA==".to_string()),
        )),
    ]);

    let accounts = client
        .get_program_ui_accounts_with_config(
            &spl_token_interface::ID,
            RpcProgramAccountsConfig {
                filters,
                account_config: RpcAccountInfoConfig {
                    encoding: Some(solana_client::rpc_config::UiAccountEncoding::Base64),
                    ..Default::default()
                },
                with_context: None,
                sort_results: None,
            },
        )
        .await?;

    match accounts.len() {
        0 => bail!("Account not found"),
        1 => Ok(accounts[0].1.clone()),
        _ => bail!("Multiple NFT accounts found"),
    }
}
