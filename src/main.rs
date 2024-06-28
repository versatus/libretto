use libretto::client::LibrettoClient;
use libretto::pubsub::FilesystemPublisher;
use libretto::watcher;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let libretto_client = LibrettoClient::new(
        "127.0.0.1:5556",
        "127.0.0.1:5555"
    ).await?;
    let event_handler = tokio::spawn(async move {
        libretto_client.run().await?;

        Ok::<(), std::io::Error>(())
    });

    let filesystem_publisher = FilesystemPublisher::new("127.0.0.1:5555").await?;
    let monitor = tokio::spawn(async move {
        let _ = watcher::monitor_directory(
            "/home/ans/projects/sandbox/test-dir/",
            filesystem_publisher
        ).await;
    });

    let _ = tokio::join!(monitor, event_handler);

    Ok(())
}
