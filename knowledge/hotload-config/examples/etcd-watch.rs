use etcd_client::{Client, WatchOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::connect(["localhost:2379"], None).await?;

    let opts = WatchOptions::new().with_watch_id(1);
    let (_, mut watch_stream) = client.watch("foo", Some(opts)).await?;

    while let Some(resp) = watch_stream.message().await? {
        // println!("{:?}", resp);
        for e in resp.events() {
            // println!("{:?}", e);
            if let Some(kv) = e.kv() {
                let v = kv.value_str()?;
                println!("{}", v);
            }
        }
    }

    Ok(())
}
