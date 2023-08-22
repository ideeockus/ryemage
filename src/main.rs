use env_logger;
use env_logger::{Builder, TimestampPrecision};
use log::LevelFilter;

use tg_controller::run_polling;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Trace)
        // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        // .filter(Some("ryemage_bot"), LevelFilter::Trace)
        .format_timestamp(Some(TimestampPrecision::Nanos))
        .init();

    run_polling().await;
}
