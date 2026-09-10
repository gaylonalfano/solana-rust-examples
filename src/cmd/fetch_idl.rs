use flate2::read::ZlibDecoder;
use solana_sdk::pubkey::Pubkey;
use std::io::Read;

use super::RpcArgs;

#[derive(clap::Args)]
pub struct Args {
    #[command(flatten)]
    rpc: RpcArgs,
    /// The program address pubkey, e.g. whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
    #[arg(long, env = "PROGRAM_ID")]
    program_id: Pubkey,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let client = self.rpc.client();
        let program_id = self.program_id;

        let (base, _) = Pubkey::find_program_address(&[], &program_id);

        let idl_address = Pubkey::create_with_seed(&base, "anchor:idl", &program_id)?;

        let idl_account_data = client.get_account_data(&idl_address).await?;

        let len = u32::from_le_bytes(idl_account_data[40..44].try_into()?);

        let mut decoder = ZlibDecoder::new(&idl_account_data[44..44 + len as usize]);
        let mut s = String::new();
        decoder.read_to_string(&mut s)?;

        let idl: serde_json::Value = serde_json::from_str(&s)?;

        println!("{:#?}", idl);

        Ok(())
    }
}
