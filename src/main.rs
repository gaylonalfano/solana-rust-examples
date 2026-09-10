mod cmd;

use clap::{Parser, Subcommand};

/// A collection of Solana scripts.
///
/// Run a command with `cargo run -- <COMMAND>`. Options can also be set with environment
/// variables (e.g. `RPC_URL`, `SIGNER_KEYPAIR`), see `<COMMAND> --help` for the names.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a new wallet and print the pubkey, Base58 private key, and JSON private key
    NewWallet,
    /// Create a new SPL token mint account
    CreateSpl(cmd::create_spl::Args),
    /// Mint SPL tokens to the associated token account of a wallet (signer must be the mint authority)
    MintSpl(cmd::mint_spl::Args),
    /// Print the balance of an associated token account, for a wallet and mint
    AssociatedTokenBalance(cmd::associated_token_balance::Args),
    /// Print the creation timestamp of an account
    CreationDate(cmd::creation_date::Args),
    /// Print the wallet address that owns an NFT
    NftOwner(cmd::nft_owner::Args),
    /// Print the mint pubkeys of every NFT in a wallet
    ListNfts(cmd::list_nfts::Args),
    /// Listen to slotSubscribe events
    Pubsub(cmd::pubsub::Args),
    /// Fetch the onchain Anchor IDL of a program
    FetchIdl(cmd::fetch_idl::Args),
    /// Transfer SOL with a memo and priority fee
    TransferSol(cmd::transfer_sol::Args),
    /// Wrap SOL into the signer's WSOL associated token account
    WrapSol(cmd::wrap_sol::Args),
    /// Print this CLI's help as Markdown, used to generate README.md
    #[command(hide = true)]
    MarkdownHelp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::NewWallet => cmd::new_wallet::run(),
        Command::CreateSpl(args) => args.run().await,
        Command::MintSpl(args) => args.run().await,
        Command::AssociatedTokenBalance(args) => args.run().await,
        Command::CreationDate(args) => args.run().await,
        Command::NftOwner(args) => args.run().await,
        Command::ListNfts(args) => args.run().await,
        Command::Pubsub(args) => args.run().await,
        Command::FetchIdl(args) => args.run().await,
        Command::TransferSol(args) => args.run().await,
        Command::WrapSol(args) => args.run().await,
        Command::MarkdownHelp => {
            let options = clap_markdown::MarkdownOptions::new()
                .title("Solana Scripts".to_string())
                .show_footer(false);
            print!("{}", clap_markdown::help_markdown_custom::<Cli>(&options));
            Ok(())
        }
    }
}
