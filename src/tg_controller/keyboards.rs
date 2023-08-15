use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, ReplyMarkup};

// pub const BASE_KEYBOARD: [[&str;3];1] = [
//     ["Build Palette", "Recolour", "Settings"]
// ];

// base actions
pub const BUILD_PALETTE: &str = "Build Palette";
pub const RECOLOUR: &str = "Recolour";
pub const SETTINGS: &str = "Settings";

// mode names
pub const SIMPLE_LAB_MODE: &str = "Luminous Transformation"; // SimpleLabMapper
pub const SIMPLE_RGB_MODE: &str = "Ruthless Chromatic"; // SimpleRgbMapper2
pub const RGB_DITHER_MODE: &str = "Mosaic Adventure"; // RgbDitherMapper
pub const NEU_QUANT_MODE: &str = "Neural Revolution"; // NeuQuantMapper
pub const RGB_SWAP_MODE: &str = "Radical Replacement"; // SwapMapper
pub const PIXEL_DIFF_MODE: &str = "Pixel Reflection"; // RgbDiffMapper

// settings buttons
pub const USER_GUIDE: &str = "User Guide";
pub const BOT_ABOUT: &str = "What is it ??";
pub const THIRD_BUTTON: &str = "Third Button";

pub fn base_keyboard() -> ReplyMarkup {
    let buttons = [
        [
            KeyboardButton::new(BUILD_PALETTE),
            KeyboardButton::new(RECOLOUR),
            KeyboardButton::new(SETTINGS),
        ]
    ];

    let mut keyboard = KeyboardMarkup::new(buttons);
    keyboard.resize_keyboard = Some(true);
    ReplyMarkup::Keyboard(keyboard)
}

pub fn recolour_mode_keyboard() -> ReplyMarkup {
    let buttons = [
        [
            InlineKeyboardButton::new(
                SIMPLE_RGB_MODE,
                InlineKeyboardButtonKind::CallbackData(SIMPLE_RGB_MODE.to_string()),
            ),
            InlineKeyboardButton::new(
                SIMPLE_LAB_MODE,
                InlineKeyboardButtonKind::CallbackData(SIMPLE_LAB_MODE.to_string()),
            ),
        ],
        [
            InlineKeyboardButton::new(
                RGB_DITHER_MODE,
                InlineKeyboardButtonKind::CallbackData(RGB_DITHER_MODE.to_string()),
            ),
            InlineKeyboardButton::new(
                RGB_SWAP_MODE,
                InlineKeyboardButtonKind::CallbackData(RGB_SWAP_MODE.to_string()),
            ),
        ],
        [
            InlineKeyboardButton::new(
                NEU_QUANT_MODE,
                InlineKeyboardButtonKind::CallbackData(NEU_QUANT_MODE.to_string()),
            ),
            InlineKeyboardButton::new(
                PIXEL_DIFF_MODE,
                InlineKeyboardButtonKind::CallbackData(PIXEL_DIFF_MODE.to_string()),
            ),
        ],
    ];

    let mut keyboard = InlineKeyboardMarkup::new(buttons);
    ReplyMarkup::InlineKeyboard(keyboard)
}

pub fn setting_keyboard() -> ReplyMarkup {
    let buttons = [
        [
            KeyboardButton::new(USER_GUIDE),
            KeyboardButton::new(BOT_ABOUT),
            KeyboardButton::new(THIRD_BUTTON),
        ]
    ];

    let mut keyboard = KeyboardMarkup::new(buttons);
    keyboard.resize_keyboard = Some(true);
    ReplyMarkup::Keyboard(keyboard)
}

pub const BOT_ABOUT_TEXT: &str = "\
Этот бот был разработан @idksdump.\
\
Ответсвенный за тексты и брендинг - ChatGPT
Дизайнер иконки - Free Logo Maker
Вдохновитель - Абстрактная рыжая девушка
";