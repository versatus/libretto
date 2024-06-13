use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use libretto::client::handle_events;
use libretto::watcher;
use notify::{RecursiveMode, Watcher};
use tokio::sync::RwLock;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    let (_stop_tx, stop_rx) = std::sync::mpsc::channel();
    let queue = Arc::new(
        RwLock::new(
            VecDeque::new()
        )
    );

    /*
    let watcher_queue = queue.clone();
    let watcher = tokio::spawn(
        async move { 
            watcher::monitor_directory("/mnt/libretto", watcher_queue.clone()).await
    });

    handle_events(queue.clone()).await;

    let _ = watcher.await?;
    */

    let event_handling_queue = queue.clone();
    tokio::spawn(async move {
        handle_events(event_handling_queue.clone()).await;
    });

    let _ = watcher::monitor_directory("/home/ans/projects/sandbox/test-dir/", queue, stop_rx).await;


    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }

    Ok(())
}
