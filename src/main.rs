
//use std::io;
use tracing::{error, warn, info, debug, trace};
// use tracing_subscriber;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};


fn main() {
    //let file_appender = tracing_appender::rolling::daily("/tmp", "cu.log");
    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // tracing_subscriber::fmt()
    //     .with_writer(io::stdout) 
    //     .with_writer(non_blocking) 
    //     .with_ansi(false)
    //     .init();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    trace!("tracing-trace");
    debug!("tracing-debug");
    info!("tracing-info");
    warn!("tracing-warn");
    error!("tracing-error");

}


