use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::net::Download;
use teloxide::prelude::*;
use crate::image_processing::PaletteMapperMode;
use crate::tg_controller::{get_downloads_dir, State};
use crate::tg_controller::keyboards::*;

pub use image_processing::*;
pub use commands::*;
pub use middleware::*;
pub use common::*;

mod image_processing;
mod commands;
mod middleware;
mod common;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

impl PaletteMapperMode {
    fn from_mode_name(value: &str) -> Option<Self> {
        match value {
            SIMPLE_LAB_MODE => Some(PaletteMapperMode::SimpleLab),
            SIMPLE_RGB_MODE => Some(PaletteMapperMode::SimpleRgb),
            RGB_DITHER_MODE => Some(PaletteMapperMode::RgbDither),
            NEU_QUANT_MODE => Some(PaletteMapperMode::NeuQuant),
            RGB_SWAP_MODE => Some(PaletteMapperMode::RgbSwap),
            PIXEL_DIFF_MODE => Some(PaletteMapperMode::PixelDiff),
            _ => None
        }
    }
}

async fn download_file_by_id(bot: &Bot, file_id: &str) -> HandlerResult {
    let tg_file = bot.get_file(file_id).await?;

    let mut async_fd = tokio::fs::File::create(get_downloads_dir().join(file_id)).await?;
    bot.download_file(&tg_file.path, &mut async_fd).await?;

    Ok(())
}


fn log_request(log_text: &str, msg: &Message) {
    log::debug!("{}", log_text);
    match msg.from() {
        None => {
            log::debug!("message from unknown user");
        }
        Some(user) => {
            log::debug!(
                "message from user {:?} [{}] - {}. special: {}|{}",
                user.mention(),
                user.id,
                user.full_name(),
                user.is_anonymous(),
                user.is_telegram(),
            );
        }
    }
}
