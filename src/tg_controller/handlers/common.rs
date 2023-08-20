use std::fmt::format;
use log::debug;
use teloxide::Bot;
use teloxide::prelude::*;

use crate::tg_controller::handlers::{HandlerResult, log_request, MyDialogue};
use crate::tg_controller::keyboards::{base_keyboard, BUILD_PALETTE};
use crate::tg_controller::ryemage_settings::UserSettings;
use crate::tg_controller::State;

pub async fn handle_start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got contact (start state) message", &msg);

    if let Some(BUILD_PALETTE) = msg.text() {
        dialogue.update(State::WaitProcessPicture { settings: UserSettings::default() }).await?;
        let mut message = bot.send_message(msg.chat.id, "Choose picture to extract palette");
        message.reply_markup = Some(base_keyboard());
        message.await?;
    }

    let mut message = bot.send_message(
        msg.chat.id,
        format!(
            "Create a palette - press on {BUILD_PALETTE}.\
            \
            Or type /help to view instruction"
        ),
    );
    message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}


pub async fn invalid_state_callback(
    bot: Bot,
    q: CallbackQuery,
) -> HandlerResult {
    debug!("got invalid callback");
    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }
    bot.answer_callback_query(q.id).await?;

    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got message, but state invalid", &msg);

    bot.send_message(msg.chat.id, "If you got stacked, please read User Guide. Just press /help").await?;

    Ok(())
}