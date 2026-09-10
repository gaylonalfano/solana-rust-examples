use solana_sdk::pubkey::Pubkey;

use super::RpcArgs;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    /// The pubkey address of the wallet that owns the tokens
    #[arg(long, env = "WALLET_PUBKEY")]
    wallet_pubkey: Pubkey,
    /// The pubkey address of the SPL Token mint account
    #[arg(long, env = "MINT_ACCOUNT_PUBKEY")]
    mint_account_pubkey: Pubkey,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let target = self.wallet_pubkey;
        let mint_id = self.mint_account_pubkey;

        let rpc = self.rpc.client();

        let addr = spl_associated_token_account_interface::address::get_associated_token_address(
            &target, &mint_id,
        );

        let balance = rpc.get_token_account_balance(&addr).await?;

        println!("Wallet pubkey: {}", target.to_string());
        println!("Mint account: {}", mint_id.to_string());
        println!("Associated token account: {}", addr.to_string());
        println!("Amount: {}", balance.ui_amount_string);
        println!("Decimals: {}", balance.decimals);

        Ok(())
    }
}
