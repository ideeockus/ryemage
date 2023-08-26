use crate::handlers::*;
use crate::ryemage_settings::UserSettings;
use crate::State;
use log::debug;
use teloxide::Bot;

pub async fn handle_start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got contact (start state) message", &msg);

    dialogue
        .update(State::WaitProcessPicture {
            settings: UserSettings::default(),
        })
        .await?;

    let mut message = bot.send_message(
        msg.chat.id,
        format!(
            r#"
Hello! I can recolor pictures reducing colors amount.

Create a palette - press on {BUILD_PALETTE}.

Type /help to view instruction"#
        ),
    );
    message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn invalid_state_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    debug!("got invalid callback");
    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }
    bot.answer_callback_query(q.id).await?;

    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got message, but state invalid", &msg);

    bot.send_message(
        msg.chat.id,
        "If you got stacked, please read User Guide. Just press /help",
    )
    .await?;

    Ok(())
}
