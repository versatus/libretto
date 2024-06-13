use notify::{Watcher, Event, RecursiveMode};
use tokio::sync::RwLock;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

pub async fn monitor_directory(path: &str, queue: Arc<RwLock<VecDeque<Event>>>) -> std::io::Result<()> {
    println!("setting up directory watcher for path: {}", path);
    let inner_queue = queue.clone();
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        let tx = tx.clone();
        match res {
            Ok(event) => {
                let _ = tx.send(event);
            },
            Err(e) => println!("watch error: {:?}", e)
        }}).map_err(|e| {
            std::io::Error::new(
            std::io::ErrorKind::Other,
            e
        )
    })?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            e
        )
    })?;

    tokio::spawn(async move {
        while let Ok(event) = rx.recv() {
            println!("Received event: {:?}", event);
            let mut guard = inner_queue.write().await;
            guard.push_back(event);
        }
    });

    println!("Directory watcher setup complete");

    Ok(())
}
