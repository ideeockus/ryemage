use teloxide::prelude::*;
use teloxide::types::ParseMode;

use crate::handlers::{log_request, HandlerResult, MyDialogue};
use crate::keyboards::{base_keyboard, BOT_HELP_TEXT_MD};
use crate::ryemage_settings::UserSettings;
use crate::State;

// currently unused
#[allow(unused)]
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got start command", &msg);

    dialogue
        .update(State::WaitProcessPicture {
            settings: UserSettings::default(),
        })
        .await?;
    let mut message = bot.send_message(msg.chat.id, "Choose action please");
    message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got help command", &msg);

    let mut message = bot.send_message(msg.chat.id, BOT_HELP_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}
