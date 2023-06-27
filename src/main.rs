
use tracing::{info};

mod log;



fn main() {
    println!("CU start !");

    let _guard = log::init();
        
    info!("CU start !");
}
