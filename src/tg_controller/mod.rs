mod commands;
mod dispatch;
mod ryemage_settings;
mod handlers;
mod keyboards;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use teloxide::prelude::*;
use teloxide::update_listeners::Polling;
use crate::tg_controller::dispatch::schema;
use teloxide::dispatching::dialogue::InMemStorage;
use crate::tg_controller::ryemage_settings::UserSettings;

const DOWNLOADED_FILES_PATH: &str = "tg_downloads";

fn get_downloads_dir() -> PathBuf {
    let tg_downloads_path = Path::new(DOWNLOADED_FILES_PATH);
    if !tg_downloads_path.exists() {
        fs::create_dir(tg_downloads_path).expect("Oops, cannot create dir");
    }

    tg_downloads_path.to_path_buf()
}

pub async fn run_bot() {
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
        .await;
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
    // WaitProcessMode {
    //     settings: UserSettings,
    // },
    ViewSettings {
        settings: UserSettings,
    },
}


pub async fn run_polling() {
    // log::info!("Preparing to run...");
    // prepare_fs();

    log::info!("Run telegram polling...");

    let bot = Bot::from_env();

    // let polling = Polling::builder(bot)
    //     .timeout(Duration::from_secs(10))
    //     .drop_pending_updates()
    //     .build();

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    log::info!("Dispatcher started");

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    //     .await;
}