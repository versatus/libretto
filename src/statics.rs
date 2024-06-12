use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref STORAGE_PATH: String = {
        dotenv::dotenv().ok();
        env::var("STORAGE_PATH").unwrap_or_else(|_| "/mnt/libretto".to_string())
    };
}
