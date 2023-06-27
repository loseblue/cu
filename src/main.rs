
use chrono::Local;
use std::io;
use tracing::{error, warn, info, debug, trace};

use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};


struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%F_%T%.3f"))
    }
}

fn main() {
    let file_appender = tracing_appender::rolling::daily("/tmp", "cu.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

     tracing_subscriber::fmt()
         .with_writer(io::stdout) 
         .with_writer(non_blocking) 
         .with_ansi(false)
         .event_format(format)
         .init();

    trace!("tracing-trace");
    debug!("tracing-debug");
    info!("tracing-info");
    warn!("tracing-warn");
    error!("tracing-error");

}


