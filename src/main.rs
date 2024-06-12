use std::collections::VecDeque;
use std::sync::Arc;

use libretto::client::handle_events;
use libretto::watcher;
use tokio::sync::RwLock;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let queue = Arc::new(
        RwLock::new(
            VecDeque::new()
        )
    );

    let watcher_queue = queue.clone();
    let watcher = tokio::spawn(
        async move { 
            watcher::monitor_directory("/mnt/libretto", watcher_queue.clone()).await
    });

    handle_events(queue.clone()).await;

    let _ = watcher.await?;

    Ok(())
}
