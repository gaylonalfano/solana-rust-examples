use solana_client::rpc_response::transaction::Transaction;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
};

use super::{RpcArgs, SignerArgs};

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    #[command(flatten)]
    signer: SignerArgs,
    /// The pubkey address of the SPL Token mint account
    #[arg(long, env = "MINT_ACCOUNT_PUBKEY")]
    mint_account_pubkey: Pubkey,
    /// The pubkey address of the wallet you want to fund with the tokens
    #[arg(long, env = "RECEIVER_PUBKEY")]
    receiver_pubkey: Pubkey,
    /// Amount to mint, in base units
    #[arg(long, default_value_t = 10_000)]
    amount: u64,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let signer_wallet: &Keypair = &self.signer.keypair;
        let client = self.rpc.client();
        let receiver_pubkey = self.receiver_pubkey;
        let mint_account_pubkey = self.mint_account_pubkey;

        let assoc = spl_associated_token_account_interface::address::get_associated_token_address(
            &receiver_pubkey,
            &mint_account_pubkey,
        );

        let assoc_instruction =
            spl_associated_token_account_interface::instruction::create_associated_token_account(
                &signer_wallet.pubkey(),
                &receiver_pubkey,
                &mint_account_pubkey,
                &spl_token_interface::ID,
            );

        let mint_to_instruction: Instruction = spl_token_interface::instruction::mint_to(
            &spl_token_interface::ID,
            &mint_account_pubkey,
            &assoc,
            &signer_wallet.pubkey(),
            &[&signer_wallet.pubkey()],
            self.amount,
        )?;

        let recent_blockhash = client.get_latest_blockhash().await?;
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[assoc_instruction, mint_to_instruction],
            Some(&signer_wallet.pubkey()),
            &[signer_wallet],
            recent_blockhash,
        );

        client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .await?;

        println!("SPL Tokens minted successfully.");
        println!("Amount: {}", self.amount);
        println!("Receiver pubkey: {}", receiver_pubkey.to_string());
        println!("Associated token account: {}", assoc.to_string());

        Ok(())
    }
}
