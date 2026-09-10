use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_response::transaction::Transaction;
use solana_sdk::signer::{Signer, keypair::Keypair};
use spl_associated_token_account_interface::instruction::create_associated_token_account_idempotent;
use spl_token_interface::instruction::sync_native;

#[derive(serde::Deserialize)]
struct Env {
    rpc_url: url::Url,
    signer_keypair: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = envy::from_env::<Env>()?;
    let signer_wallet = Keypair::from_base58_string(&env.signer_keypair);
    println!("Signer: {}", signer_wallet.pubkey().to_string());
    let client = RpcClient::new(env.rpc_url.to_string());

    // Native SOL mint (WSOL)
    let native_mint = spl_token_interface::native_mint::ID;

    // Get or create associated token account for WSOL
    let associated_token_account =
        spl_associated_token_account_interface::address::get_associated_token_address(
            &signer_wallet.pubkey(),
            &native_mint,
        );

    // Amount of SOL to wrap (in lamports)
    let amount = 1 * 1_000_000_000;

    let mut instructions = vec![];

    // Create associated token account if it doesn't exist
    instructions.push(create_associated_token_account_idempotent(
        &signer_wallet.pubkey(),
        &signer_wallet.pubkey(),
        &native_mint,
        &spl_token_interface::ID,
    ));

    // Transfer SOL to the WSOL account
    instructions.push(solana_system_interface::instruction::transfer(
        &signer_wallet.pubkey(),
        &associated_token_account,
        amount,
    ));

    // Sync native - this converts the SOL to WSOL
    instructions.push(sync_native(
        &spl_token_interface::ID,
        &associated_token_account,
    )?);

    let recent_blockhash = client.get_latest_blockhash().await?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&signer_wallet.pubkey()),
        &[&signer_wallet],
        recent_blockhash,
    );

    let sig = client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .await?;

    println!("Amount: {} lamports", amount);
    println!("WSOL account: {}", associated_token_account);
    println!("Signature: {}", sig);

    Ok(())
}
