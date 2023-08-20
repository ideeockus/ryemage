use log::{debug, error};

use teloxide::prelude::*;
use teloxide::types::{ChatAction, InputFile, ParseMode};

use crate::image_processing::{perform_action_on_files, PaletteMapperMode};
use crate::tg_controller::{get_downloads_dir, State};

use crate::tg_controller::handlers::{download_file_by_id, log_request, HandlerResult, MyDialogue};
use crate::tg_controller::keyboards::*;
use crate::tg_controller::ryemage_settings::UserSettings;

pub async fn handle_base_action(
    bot: Bot,
    dialogue: MyDialogue,
    settings: UserSettings,
    msg: Message,
) -> HandlerResult {
    log_request("got base action", &msg);

    if let Some(photo_size) = msg.photo().map(|sizes| sizes.last()).flatten() {
        // if got photo, update state and send inline keyboard to choose modes
        let file_id = photo_size.file.id.clone();
        download_file_by_id(&bot, &file_id).await?;

        dialogue
            .update(State::WaitProcessPicture {
                settings: UserSettings {
                    process_file_id: Some(file_id),
                    palette_file_id: settings.palette_file_id,
                },
            })
            .await?;

        let mut message = bot.send_message(msg.chat.id, "Great! Now make your choice.");
        message.reply_markup = Some(recolour_mode_keyboard());
        message.await?;
        return Ok(());
    }

    match msg.text() {
        Some(BUILD_PALETTE) => {
            dialogue
                .update(State::WaitPalettePicture { settings })
                .await?;
            let mut message = bot.send_message(
                msg.chat.id,
                "Ok, now you can send me picture to extract palette",
            );
            message.reply_markup = Some(back_keyboard());
            message.await?;
        }
        Some(RECOLOUR) => {
            bot.send_message(msg.chat.id, "Just send me picture.")
                .await?;
        }
        Some(SETTINGS) => {
            dialogue.update(State::ViewSettings { settings }).await?;
            let mut message = bot.send_message(msg.chat.id, "Settings");
            message.reply_markup = Some(setting_keyboard());
            message.await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "I don't understand you 😟")
                .await?; // yes, i hardcoded emoji
        }
    }

    Ok(())
}

pub async fn handle_palette_image(
    bot: Bot,
    dialogue: MyDialogue,
    settings: UserSettings,
    msg: Message,
) -> HandlerResult {
    log_request("got palette image", &msg);

    if let Some(photo_size) = msg.photo().map(|sizes| sizes.last()).flatten() {
        let file_id = photo_size.file.id.clone();
        download_file_by_id(&bot, &file_id).await?;

        dialogue
            .update(State::WaitProcessPicture {
                settings: UserSettings {
                    process_file_id: settings.process_file_id,
                    palette_file_id: Some(file_id),
                },
            })
            .await?;

        let mut message = bot.send_message(msg.chat.id, "Palette extracted");
        message.reply_markup = Some(base_keyboard());
        message.await?;

        return Ok(());
    }

    match msg.text() {
        Some(BACK) => {
            let mut message = bot.send_message(msg.chat.id, "Ok, go back");
            dialogue
                .update(State::WaitProcessPicture { settings })
                .await?;
            message.reply_markup = Some(base_keyboard());
            message.await?;
        }
        _ => {
            bot.send_message(
                msg.chat.id,
                "My current state - waiting picture to palette extraction",
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn handle_process_mode(
    bot: Bot,
    settings: UserSettings,
    q: CallbackQuery,
) -> HandlerResult {
    debug!("got process image");
    bot.answer_callback_query(q.id).await?;
    bot.send_chat_action(q.from.id, ChatAction::UploadPhoto)
        .await?;

    let (process_file_id, palette_file_id) = match settings {
        UserSettings {
            process_file_id: Some(process_file_id),
            palette_file_id: Some(palette_file_id),
        } => (process_file_id, palette_file_id),
        UserSettings {
            process_file_id,
            palette_file_id,
        } => {
            if process_file_id.is_none() {
                bot.send_message(q.from.id, "Hmmm. Seems i can't find file to process")
                    .await?;
            }
            if palette_file_id.is_none() {
                bot.send_message(
                    q.from.id,
                    "Hmmm. Did you press button with sign \"Build Palette\"?",
                )
                .await?;
            }

            if let Some(msg) = q.message {
                bot.delete_message(q.from.id, msg.id).await?;
            }

            return Ok(());
        }
    };

    match q.data.as_deref() {
        Some(mode @ PIXEL_DIFF_MODE) | Some(mode @ NEU_QUANT_MODE) => {
            bot.send_message(
                q.from.id,
                format!("Mode {mode} in development stage. Try a bit later.."),
            )
            .await?;
            bot.send_message(
                q.from.id,
                format!("I see you have uploaded file {process_file_id} and have palette"),
            )
            .await?;
        }
        Some(mode) => {
            let palette_file_name = get_downloads_dir().join(palette_file_id);
            let process_file_name = get_downloads_dir().join(process_file_id);

            let mode = match PaletteMapperMode::from_mode_name(mode) {
                None => {
                    bot.send_message(q.from.id, "Unknown mode, contact the developer")
                        .await?;
                    return Ok(());
                }
                Some(mode) => mode,
            };

            let processed = perform_action_on_files(&palette_file_name, &process_file_name, mode);

            match processed {
                Ok(v) => {
                    bot.send_photo(q.from.id, InputFile::memory(v)).await?;
                }
                Err(err) => {
                    error!("Image processing error {}", err);

                    bot.send_message(
                        q.from.id,
                        "Error occurred during image processing. Please contact the developer",
                    )
                    .await?;
                }
            }
        }
        _ => {
            bot.send_message(q.from.id, "Something goes wrong").await?;
        }
    }

    Ok(())
}

pub async fn view_settings(
    bot: Bot,
    dialogue: MyDialogue,
    settings: UserSettings,
    msg: Message,
) -> HandlerResult {
    log_request("got view settings request", &msg);

    match msg.text() {
        Some(BACK) => {
            dialogue
                .update(State::WaitProcessPicture { settings })
                .await?;
            let mut message = bot.send_message(msg.chat.id, "Choose your action");
            message.reply_markup = Some(base_keyboard());
            message.await?;
        }
        Some(BOT_ABOUT) => {
            bot.send_message(msg.chat.id, BOT_ABOUT_TEXT_MD)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        Some(USER_GUIDE) => {
            bot.send_message(
                msg.chat.id,
                "There is nothing here. Wait for the rye release.",
            )
            .await?;
        }
        Some(THIRD_BUTTON) => {
            bot.send_message(msg.chat.id, "42 - 3 = 20").await?;
        }
        _ => {
            bot.send_message(
                msg.chat.id,
                "It would be better if you pressed the third button than what you are doing now",
            )
            .await?;
        }
    }

    Ok(())
}
