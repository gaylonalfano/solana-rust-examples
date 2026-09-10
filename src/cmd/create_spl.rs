use solana_client::rpc_response::transaction::Transaction;
use solana_sdk::{
    instruction::Instruction,
    program_pack::Pack,
    signer::{Signer, keypair::Keypair},
};
use spl_token_interface::state::Mint;
use std::sync::Arc;

use super::{RpcArgs, SignerArgs, parse_keypair};

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    #[command(flatten)]
    signer: SignerArgs,
    /// Base58 encoded keypair, representing the new mint account
    #[arg(long, env = "MINT_KEYPAIR", hide_env_values = true, value_parser = parse_keypair)]
    mint_keypair: Arc<Keypair>,
    /// Number of decimals for the new mint
    #[arg(long, default_value_t = 9)]
    decimals: u8,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let signer_wallet: &Keypair = &self.signer.keypair;
        let mint_account: &Keypair = &self.mint_keypair;
        let client = self.rpc.client();

        let minimum_balance_for_rent_exemption = client
            .get_minimum_balance_for_rent_exemption(Mint::LEN)
            .await?;

        let create_account_instruction: Instruction =
            solana_system_interface::instruction::create_account(
                &signer_wallet.pubkey(),
                &mint_account.pubkey(),
                minimum_balance_for_rent_exemption,
                Mint::LEN as u64,
                &spl_token_interface::ID,
            );

        let initialize_mint_instruction: Instruction =
            spl_token_interface::instruction::initialize_mint(
                &spl_token_interface::ID,
                &mint_account.pubkey(),
                &signer_wallet.pubkey(),
                None,
                self.decimals,
            )?;

        let recent_blockhash = client.get_latest_blockhash().await?;

        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[create_account_instruction, initialize_mint_instruction],
            Some(&signer_wallet.pubkey()),
            &[mint_account, signer_wallet],
            recent_blockhash,
        );

        client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .await?;

        println!(
            "SPL Token mint account with {} decimals created successfully:\n{}",
            self.decimals,
            mint_account.pubkey().to_string()
        );

        Ok(())
    }
}
