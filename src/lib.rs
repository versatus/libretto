pub mod client;
pub mod server;
pub mod watcher;
pub mod statics;
pub mod pubsub;

pub mod dfs {
    tonic::include_proto!("dfs");
}
