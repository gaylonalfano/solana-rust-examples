use solana_client::nonblocking::rpc_client::RpcClient;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_sdk::{
    signer::{Signer, keypair::Keypair},
    transaction::{Transaction, VersionedTransaction},
};

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair: String,
    receiver_pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let signer_wallet = Keypair::from_base58_string(&env.signer_keypair);
    let client = RpcClient::new(env.rpc_url.to_string());
    let receiver_pubkey = env.receiver_pubkey.parse()?;

    let amount = 1_000_000;
    let memo = "hello solana";

    // Priority fee = compute unit limit * compute unit price (in micro-lamports).
    let compute_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(50_000);
    let compute_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1_000);

    let transfer_ix = solana_system_interface::instruction::transfer(
        &signer_wallet.pubkey(),
        &receiver_pubkey,
        amount,
    );

    let memo_ix = spl_memo_interface::instruction::build_memo(
        &spl_memo_interface::v3::ID,
        memo.as_bytes(),
        &[&signer_wallet.pubkey()],
    );

    let recent_blockhash = client.get_latest_blockhash().await?;
    let transaction: Transaction = Transaction::new_signed_with_payer(
        &[compute_limit_ix, compute_price_ix, transfer_ix, memo_ix],
        Some(&signer_wallet.pubkey()),
        &[&signer_wallet],
        recent_blockhash,
    );

    let vt = VersionedTransaction::from(transaction);

    let sig = client
        .send_and_confirm_transaction_with_spinner(&vt)
        .await?;

    println!("SOL sent.");
    println!("Amount: {}", amount);
    println!("Receiver pubkey: {}", receiver_pubkey.to_string());
    println!("Memo: {}", memo);
    println!("Signature: {}", sig.to_string());

    Ok(())
}
