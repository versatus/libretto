use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

use libretto::client::handle_events;
use libretto::watcher;
use notify::{RecursiveMode, Watcher};
use tokio::sync::RwLock;


#[tokio::main]
async fn main() -> std::io::Result<()> {

    /*
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e)
        }
    }).unwrap();
    
    watcher.watch(Path::new("/mnt/libretto"), RecursiveMode::Recursive).unwrap();
    */

    
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

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60));
    }

    Ok(())
}
