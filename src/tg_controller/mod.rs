use std::fs;
use std::path::{Path, PathBuf};

use log::info;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

use crate::tg_controller::dispatch::schema;
use crate::tg_controller::ryemage_settings::UserSettings;

mod commands;
mod dispatch;
mod handlers;
mod keyboards;
mod ryemage_settings;

const DOWNLOADED_FILES_PATH: &str = "tg_downloads";

fn get_downloads_dir() -> PathBuf {
    let tg_downloads_path = Path::new(DOWNLOADED_FILES_PATH);
    if !tg_downloads_path.exists() {
        fs::create_dir(tg_downloads_path).expect("Oops, cannot create dir");
    }

    tg_downloads_path.to_path_buf()
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    WaitProcessPicture {
        settings: UserSettings,
    },
    WaitPalettePicture {
        settings: UserSettings,
    },
    ViewSettings {
        settings: UserSettings,
    },
}

pub async fn run_polling() {
    info!("Run telegram polling...");

    let bot = Bot::from_env();

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}
