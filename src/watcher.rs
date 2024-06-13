use notify::{Watcher, Event, RecursiveMode};
use tokio::sync::RwLock;
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

pub async fn monitor_directory(
    path: &str,
    queue: Arc<RwLock<VecDeque<Event>>>,
    stop_recv: std::sync::mpsc::Receiver<()>
) -> std::io::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<Event>();

    let mut watcher = notify::recommended_watcher(move |res| {
        let tx = tx.clone();
        match res {
            Ok(event) => {
                let _ = tx.send(event);
            }
            Err(e) => println!("watch error: {:?}", e)
        }
    }).unwrap();


    watcher.watch(Path::new(path), RecursiveMode::Recursive).unwrap();

    let inner_queue = queue.clone();
    tokio::spawn(async move {
        loop {
            match rx.recv() {
                Ok(event) => {
                    let mut guard = inner_queue.write().await;
                    guard.push_back(event.clone());
                    drop(guard);
                }
                Err(e) => {
                    println!("Error: {e}");
                    break
                }
            }
        }
        println!("Channel closed");
    });



    loop {
        if let Ok(()) = stop_recv.recv() {
            break
        }
        std::thread::sleep(Duration::from_secs(60));
    }

    Ok(())
}
