# Solana Scripts

This document contains the help content for the `scripts` command-line program.

**Command Overview:**

* [`scripts`↴](#scripts)
* [`scripts new-wallet`↴](#scripts-new-wallet)
* [`scripts create-spl`↴](#scripts-create-spl)
* [`scripts mint-spl`↴](#scripts-mint-spl)
* [`scripts associated-token-balance`↴](#scripts-associated-token-balance)
* [`scripts creation-date`↴](#scripts-creation-date)
* [`scripts nft-owner`↴](#scripts-nft-owner)
* [`scripts list-nfts`↴](#scripts-list-nfts)
* [`scripts pubsub`↴](#scripts-pubsub)
* [`scripts fetch-idl`↴](#scripts-fetch-idl)
* [`scripts transfer-sol`↴](#scripts-transfer-sol)
* [`scripts wrap-sol`↴](#scripts-wrap-sol)

## `scripts`

A collection of Solana scripts.

Run a command with `cargo run -- <COMMAND>`. Options can also be set with environment variables (e.g. `RPC_URL`, `SIGNER_KEYPAIR`), see `<COMMAND> --help` for the names.

**Usage:** `scripts <COMMAND>`

###### **Subcommands:**

* `new-wallet` — Generate a new wallet and print the pubkey, Base58 private key, and JSON private key
* `create-spl` — Create a new SPL token mint account
* `mint-spl` — Mint SPL tokens to the associated token account of a wallet (signer must be the mint authority)
* `associated-token-balance` — Print the balance of an associated token account, for a wallet and mint
* `creation-date` — Print the creation timestamp of an account
* `nft-owner` — Print the wallet address that owns an NFT
* `list-nfts` — Print the mint pubkeys of every NFT in a wallet
* `pubsub` — Listen to slotSubscribe events
* `fetch-idl` — Fetch the onchain Anchor IDL of a program
* `transfer-sol` — Transfer SOL with a memo and priority fee
* `wrap-sol` — Wrap SOL into the signer's WSOL associated token account



## `scripts new-wallet`

Generate a new wallet and print the pubkey, Base58 private key, and JSON private key

**Usage:** `scripts new-wallet`



## `scripts create-spl`

Create a new SPL token mint account

**Usage:** `scripts create-spl [OPTIONS] --rpc-url <RPC_URL> --signer-keypair <KEYPAIR> --mint-keypair <MINT_KEYPAIR>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--signer-keypair <KEYPAIR>` — Base58 encoded keypair that signs and pays for the transaction
* `--mint-keypair <MINT_KEYPAIR>` — Base58 encoded keypair, representing the new mint account
* `--decimals <DECIMALS>` — Number of decimals for the new mint

  Default value: `9`



## `scripts mint-spl`

Mint SPL tokens to the associated token account of a wallet (signer must be the mint authority)

**Usage:** `scripts mint-spl [OPTIONS] --rpc-url <RPC_URL> --signer-keypair <KEYPAIR> --mint-account-pubkey <MINT_ACCOUNT_PUBKEY> --receiver-pubkey <RECEIVER_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--signer-keypair <KEYPAIR>` — Base58 encoded keypair that signs and pays for the transaction
* `--mint-account-pubkey <MINT_ACCOUNT_PUBKEY>` — The pubkey address of the SPL Token mint account
* `--receiver-pubkey <RECEIVER_PUBKEY>` — The pubkey address of the wallet you want to fund with the tokens
* `--amount <AMOUNT>` — Amount to mint, in base units

  Default value: `10000`



## `scripts associated-token-balance`

Print the balance of an associated token account, for a wallet and mint

**Usage:** `scripts associated-token-balance --rpc-url <RPC_URL> --wallet-pubkey <WALLET_PUBKEY> --mint-account-pubkey <MINT_ACCOUNT_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--wallet-pubkey <WALLET_PUBKEY>` — The pubkey address of the wallet that owns the tokens
* `--mint-account-pubkey <MINT_ACCOUNT_PUBKEY>` — The pubkey address of the SPL Token mint account



## `scripts creation-date`

Print the creation timestamp of an account

**Usage:** `scripts creation-date --rpc-url <RPC_URL> --account-pubkey <ACCOUNT_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--account-pubkey <ACCOUNT_PUBKEY>` — The pubkey address of the account you want to introspect



## `scripts nft-owner`

Print the wallet address that owns an NFT

**Usage:** `scripts nft-owner --rpc-url <RPC_URL> --mint-account-pubkey <MINT_ACCOUNT_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--mint-account-pubkey <MINT_ACCOUNT_PUBKEY>` — The pubkey address of the SPL Token mint account



## `scripts list-nfts`

Print the mint pubkeys of every NFT in a wallet

**Usage:** `scripts list-nfts --rpc-url <RPC_URL> --wallet-pubkey <WALLET_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--wallet-pubkey <WALLET_PUBKEY>` — The pubkey address of the wallet that owns the NFTs



## `scripts pubsub`

Listen to slotSubscribe events

**Usage:** `scripts pubsub [OPTIONS] --ws-url <WS_URL>`

###### **Options:**

* `--ws-url <WS_URL>` — Solana websocket endpoint, e.g. wss://api.mainnet-beta.solana.com
* `--count <COUNT>` — Number of slot updates to print before exiting

  Default value: `5`



## `scripts fetch-idl`

Fetch the onchain Anchor IDL of a program

**Usage:** `scripts fetch-idl --rpc-url <RPC_URL> --program-id <PROGRAM_ID>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--program-id <PROGRAM_ID>` — The program address pubkey, e.g. whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc



## `scripts transfer-sol`

Transfer SOL with a memo and priority fee

**Usage:** `scripts transfer-sol [OPTIONS] --rpc-url <RPC_URL> --signer-keypair <KEYPAIR> --receiver-pubkey <RECEIVER_PUBKEY>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--signer-keypair <KEYPAIR>` — Base58 encoded keypair that signs and pays for the transaction
* `--receiver-pubkey <RECEIVER_PUBKEY>` — The pubkey address of the wallet receiving the SOL
* `--amount <AMOUNT>` — Amount to send, in lamports

  Default value: `1000000`
* `--memo <MEMO>` — Memo attached to the transfer

  Default value: `hello solana`



## `scripts wrap-sol`

Wrap SOL into the signer's WSOL associated token account

**Usage:** `scripts wrap-sol [OPTIONS] --rpc-url <RPC_URL> --signer-keypair <KEYPAIR>`

###### **Options:**

* `--rpc-url <RPC_URL>` — Solana RPC endpoint, e.g. https://api.mainnet-beta.solana.com
* `--signer-keypair <KEYPAIR>` — Base58 encoded keypair that signs and pays for the transaction
* `--amount <AMOUNT>` — Amount of SOL to wrap, in lamports

  Default value: `1000000000`



