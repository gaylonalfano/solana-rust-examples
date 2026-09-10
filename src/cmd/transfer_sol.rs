use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair},
    transaction::{Transaction, VersionedTransaction},
};

use super::{RpcArgs, SignerArgs};

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    #[command(flatten)]
    signer: SignerArgs,
    /// The pubkey address of the wallet receiving the SOL
    #[arg(long, env = "RECEIVER_PUBKEY")]
    receiver_pubkey: Pubkey,
    /// Amount to send, in lamports
    #[arg(long, default_value_t = 1_000_000)]
    amount: u64,
    /// Memo attached to the transfer
    #[arg(long, default_value = "hello solana")]
    memo: String,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let signer_wallet: &Keypair = &self.signer.keypair;
        let client = self.rpc.client();
        let receiver_pubkey = self.receiver_pubkey;

        // Priority fee = compute unit limit * compute unit price (in micro-lamports).
        let compute_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(50_000);
        let compute_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1_000);

        let transfer_ix = solana_system_interface::instruction::transfer(
            &signer_wallet.pubkey(),
            &receiver_pubkey,
            self.amount,
        );

        let memo_ix = spl_memo_interface::instruction::build_memo(
            &spl_memo_interface::v3::ID,
            self.memo.as_bytes(),
            &[&signer_wallet.pubkey()],
        );

        let recent_blockhash = client.get_latest_blockhash().await?;
        let transaction: Transaction = Transaction::new_signed_with_payer(
            &[compute_limit_ix, compute_price_ix, transfer_ix, memo_ix],
            Some(&signer_wallet.pubkey()),
            &[signer_wallet],
            recent_blockhash,
        );

        let vt = VersionedTransaction::from(transaction);

        let sig = client
            .send_and_confirm_transaction_with_spinner(&vt)
            .await?;

        println!("SOL sent.");
        println!("Amount: {}", self.amount);
        println!("Receiver pubkey: {}", receiver_pubkey.to_string());
        println!("Memo: {}", self.memo);
        println!("Signature: {}", sig.to_string());

        Ok(())
    }
}
