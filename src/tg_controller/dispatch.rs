use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::dptree::endpoint;
use teloxide::prelude::*;
use tokio::io::AsyncReadExt;
use crate::tg_controller::commands::Command;
use crate::tg_controller::handlers::*;
use crate::tg_controller::ryemage_settings::UserSettings;
use crate::tg_controller::State;

// #[derive(Clone, Default)]
// pub enum State {
//     #[default]
//     Start,
//     ReceiveFullName,
//     ReceiveProductChoice {
//         full_name: String,
//     },
// }
//
// pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//     use dptree::case;
//
//     let command_handler = teloxide::filter_command::<Command, _>()
//         .branch(
//             case![State::Start]
//                 .branch(case![Command::Help].endpoint(help))
//                 .branch(case![Command::Start].endpoint(start)),
//         )
//         .branch(case![Command::Cancel].endpoint(cancel));
//
//     let message_handler = Update::filter_message()
//         .branch(command_handler)
//         .branch(case![State::ReceiveFullName].endpoint(receive_full_name))
//         .branch(dptree::endpoint(invalid_state));
//
//     let callback_query_handler = Update::filter_callback_query().branch(
//         case![State::ReceiveProductChoice { full_name }].endpoint(receive_product_selection),
//     );
//
//     dialogue::enter::<Update, InMemStorage<State>, State, _>()
//         .branch(message_handler)
//         .branch(callback_query_handler)
// }
//
// type MyDialogue = Dialogue<State, InMemStorage<State>>;
// type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//
// async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     todo!()
// }
// async fn help(bot: Bot, msg: Message) -> HandlerResult {
//     todo!()
// }
// async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     todo!()
// }
// async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
//     todo!()
// }
// async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     todo!()
// }
// async fn receive_product_selection(
//     bot: Bot,
//     dialogue: MyDialogue,
//     full_name: String, // Available from `State::ReceiveProductChoice`.
//     q: CallbackQuery,
// ) -> HandlerResult {
//     todo!()
// }

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start));
    // .branch(
    //     case![State::Start]
    //         .branch(case![Command::Help].endpoint(help))
    //         .branch(case![Command::Start].endpoint(start)),
    // );

    let message_handler = Update::filter_message()
        // TODO add logging middleware
        // .chain({
        //     teloxide::dptree::endpoint(log_request)
        //     ControlFlow::Continue()
        // })
        // .branch(
        //     dptree::entry()
        //         .endpoint(log_request).map()
        // )
        .branch(command_handler)
        .branch(case![State::WaitProcessPicture { settings }].endpoint(handle_base_action))
        .branch(case![State::WaitPalettePicture { settings }]
            .endpoint(handle_palette_image))
        // .branch(case![State::WaitProcessPicture { settings }]
        //     .endpoint(handle_process_image))
        .branch(case![State::ViewSettings { settings }]
            .endpoint(view_settings))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(
            case![State::WaitProcessPicture { settings }].endpoint(handle_process_mode),
        )
        .branch(dptree::endpoint(invalid_state_callback));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
