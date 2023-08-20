use teloxide::prelude::*;
use crate::tg_controller::handlers::{HandlerResult, log_request, MyDialogue};
use crate::tg_controller::keyboards::base_keyboard;
use crate::tg_controller::ryemage_settings::UserSettings;
use crate::tg_controller::State;

// currently unused
#[allow(unused)]
pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got start command", &msg);

    dialogue.update(State::WaitProcessPicture { settings: UserSettings::default() }).await?;
    let mut message = bot.send_message(msg.chat.id, "Choose action please");
    message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got help command", &msg);

    bot.send_message(msg.chat.id, "How can I help you?").await?;

    Ok(())
}
