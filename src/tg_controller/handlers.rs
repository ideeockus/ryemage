use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::payloads::SendMessage;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup, Me, ReplyMarkup, User};

use crate::tg_controller::commands::Command;
use crate::tg_controller::keyboards::*;
use crate::tg_controller::reymage_settings::UserSettings;
use crate::tg_controller::State;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn log_request(msg: Message) -> HandlerResult {
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

    Ok(())
}

pub async fn start(bot: Bot, msg: Message) -> HandlerResult {
    log::debug!("got start command");

    // let buttons = &[
    //     &["Build Palette", "Recolour", "Settings"]
    // ];

    // let to_kb = |btn: &[&[&str]]| {
    //     btn.into_iter().map(move |row| row.into_iter().map(move |btn_text| {
    //         KeyboardButton::new(*btn_text)
    //     }))
    // };

    let mut message = bot.send_message(msg.chat.id, "Choose action please");
    message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    log::debug!("got help command");

    Ok(())
}

pub async fn handle_base_action(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log::debug!("got base action");

    match msg.text() {
        Some(BUILD_PALETTE) => {
            dialogue.update(State::WaitPalettePicture { settings: UserSettings::default() });
            bot.send_message(msg.chat.id, "Ok, now you can send me picture to extract palette").await?;
        }
        Some(RECOLOUR) => {
            dialogue.update(State::WaitProcessMode { settings: UserSettings::default() });
            let mut message = bot.send_message(msg.chat.id, "Ok, now you can send me picture to extract palette");
            message.reply_markup = Some(recolour_mode_keyboard());
            message.await?;
        }
        Some(SETTINGS) => {
            dialogue.update(State::ViewSettings { settings: UserSettings::default() });
            let mut message = bot.send_message(msg.chat.id, "Settings");
            message.reply_markup = Some(setting_keyboard());
            message.await?;
        }
        _ => {

        }
    }

    Ok(())
}

pub async fn handle_palette_image(bot: Bot, settings: UserSettings, msg: Message) -> HandlerResult {
    log::debug!("got palette image");

    Ok(())
}

pub async fn handle_process_image(bot: Bot, settings: UserSettings, msg: Message) -> HandlerResult {
    log::debug!("got process image");

    Ok(())
}

pub async fn handle_process_mode(
    bot: Bot,
    settings: UserSettings,
    q: CallbackQuery,
) -> HandlerResult {
    log::debug!("got process image");

    match q.data {
        Some(t) => {
            bot.send_message(
                q.from.id,
                format!("Mode {t} in development stage. Try a bit later.."),
            ).await?;
        }
        _ => {
            bot.send_message(q.from.id, "Something goes wrong").await?;
        }
    }

    Ok(())
}


pub async fn view_settings(bot: Bot, settings: UserSettings, msg: Message) -> HandlerResult {
    log::debug!("got view settings request");

    match msg.text() {
        Some(BOT_ABOUT) => {
            bot.send_message(msg.chat.id, BOT_ABOUT_TEXT).await?;
        }
        Some(USER_GUIDE) => {
            bot.send_message(msg.chat.id, "Тут пока пусто, но инструкция будет").await?;
        }
        Some(THIRD_BUTTON) => {
            bot.send_message(msg.chat.id, "Почему рожь? Ответ").await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Ты что-то не то нажал сейчас").await?;
        }
    }

    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    log::debug!("got message, but state invalid");

    Ok(())
}
