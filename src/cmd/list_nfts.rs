use anyhow::{Context, bail};
use solana_account_decoder::{UiAccountData, parse_token::UiTokenAccount};
use solana_sdk::pubkey::Pubkey;

use super::RpcArgs;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    /// The pubkey address of the wallet that owns the NFTs
    #[arg(long, env = "WALLET_PUBKEY")]
    wallet_pubkey: Pubkey,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = self.rpc.client();
        let wallet = self.wallet_pubkey;

        let token_accounts = client
            .get_token_accounts_by_owner(
                &wallet,
                solana_client::rpc_request::TokenAccountsFilter::ProgramId(spl_token_interface::ID),
            )
            .await?;

        let parsed_accounts = token_accounts
            .iter()
            .map(|token_account| {
                let UiAccountData::Json(json_data) = &token_account.account.data else {
                    bail!("non-JSON token account data returned");
                };
                let info = json_data
                    .parsed
                    .get("info")
                    .context("missing 'info' field")?;
                let data = serde_json::from_value::<UiTokenAccount>(info.clone())?;
                Ok(data)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let nfts: Vec<_> = parsed_accounts
            .into_iter()
            .filter(|acct| {
                acct.token_amount.decimals == 0 && acct.token_amount.ui_amount == Some(1.0)
            })
            .map(|acct| acct.mint)
            .collect();

        println!("NFTs owned by {}:", wallet.to_string());
        println!("{:#?}", nfts);

        Ok(())
    }
}
