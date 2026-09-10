use anyhow::Context;
use chrono::prelude::*;
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::{
    str::FromStr,
    time::{Duration, UNIX_EPOCH},
};

use super::RpcArgs;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    /// The pubkey address of the account you want to introspect
    #[arg(long, env = "ACCOUNT_PUBKEY")]
    account_pubkey: Pubkey,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let rpc = self.rpc.client();
        let addr = self.account_pubkey;

        let datetime = get_account_creation_date(&rpc, &addr).await?;

        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{} creation date:", addr.to_string());
        println!("UTC - {}", timestamp_str);

        Ok(())
    }
}

async fn get_account_creation_date(
    rpc: &RpcClient,
    addr: &Pubkey,
) -> anyhow::Result<DateTime<Utc>> {
    #[async_recursion::async_recursion]
    async fn fetch(
        rpc: &RpcClient,
        addr: &Pubkey,
        before: Option<Signature>,
    ) -> anyhow::Result<RpcConfirmedTransactionStatusWithSignature> {
        let mut sigs = rpc
            .get_signatures_for_address_with_config(
                &addr,
                solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config {
                    before,
                    ..Default::default()
                },
            )
            .await?;

        sigs.sort_by_key(|sig| sig.block_time);

        let earliest = sigs.first().context("Empty signature list!")?;

        if sigs.len() < 1000 {
            Ok(earliest.clone())
        } else {
            let sig = Signature::from_str(&earliest.signature)?;
            fetch(&rpc, &addr, Some(sig)).await
        }
    }

    let status = fetch(&rpc, &addr, None).await?;

    let d = UNIX_EPOCH
        + Duration::from_secs(
            status
                .block_time
                .context("Missing block time!")?
                .try_into()?,
        );

    Ok(DateTime::<Utc>::from(d))
}
