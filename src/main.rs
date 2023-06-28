
use tracing::{info};
mod log;
use tokio;



fn main() {
    println!("CU start !");
    info!("CU start !");

    let _guard = log::init();
        
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

        println!("CU end !");
}

