use tg_controller::run_bot;
use crate::tg_controller::run_polling;

mod image_processing;
mod tg_controller;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    run_polling().await;
}
