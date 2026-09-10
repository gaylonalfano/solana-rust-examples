use futures_util::StreamExt;
use solana_client::nonblocking::pubsub_client::PubsubClient;

#[derive(clap::Args)]
pub struct Args {
    /// Solana websocket endpoint, e.g. wss://api.mainnet-beta.solana.com
    #[arg(long, env = "WS_URL")]
    ws_url: url::Url,
    /// Number of slot updates to print before exiting
    #[arg(long, default_value_t = 5)]
    count: usize,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        let ps_client = PubsubClient::new(self.ws_url.as_str()).await?;

        let (mut slots, unsubscriber) = ps_client.slot_subscribe().await?;

        let mut count = 0;
        while let Some(response) = slots.next().await {
            println!("{:?}", response);
            count += 1;
            if count >= self.count {
                break;
            }
        }

        unsubscriber().await;

        Ok(())
    }
}
