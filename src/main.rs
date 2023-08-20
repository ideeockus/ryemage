use pretty_env_logger;
use tg_controller::run_polling;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    run_polling().await;
}
