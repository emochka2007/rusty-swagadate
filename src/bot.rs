use crate::profile::Profile;
use log::{debug, info};
use teloxide::dispatching::dialogue::{GetChatId, Storage};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};
use teloxide::sugar::bot::BotMessagesExt;
use teloxide::types::{InlineQueryResultArticle, InputMessageContent, InputMessageContentText, KeyboardButton, KeyboardMarkup, Me};

pub struct SwagaBot {
    bot: Bot,
}
#[derive(Clone, Default, Debug)]
pub enum State {
    #[default]
    Start,
    Profile {
        username: String,
    },
    ListOptions,
}
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// These commands are supported:
#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text
    Help,
    /// Start
    Start,
}

type MyDialogue = Arc<InMemStorage<State>>;

impl SwagaBot {
    /// Parse the text wrote on Telegram and check if that text is a valid command
    /// or not, then match the command. If the command is `/start` it writes a
    /// markup with the `InlineKeyboardMarkup`.
    async fn message_handler(bot: Bot, dialogue: MyDialogue,   msg: Message, me: Me) -> HandlerResult {
        let text = msg.text().unwrap();
        if let Some(state) = dialogue.get_dialogue(msg.chat_id().unwrap()).await? {
            info!("{:?}",state);
        }
        if let Some(text) = msg.text() {
            let is_command = text.starts_with("/");
            if is_command {
                match BotCommands::parse(text, me.username()) {
                    Ok(Command::Help) => {
                        SwagaBot::start(bot, msg).await?;
                    }
                    Ok(Command::Start) => {
                        SwagaBot::start(bot, msg).await?;
                    }
                    Err(_) => {
                        bot.send_message(msg.chat.id, "Command not found!").await?;
                    }
                }
            } else {
                match text {
                    "1" => {
                        info!("{text}");
                    }
                    "2" => {
                        Self::refresh_profile(&bot, msg.chat_id().unwrap()).await?;
                        info!("{text}");
                    }
                    "3" => {
                        info!("{text}");
                    }
                    "4" => {
                        info!("{text}");
                    }
                    _ => {
                        info!("Unknown func");
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn refresh_profile(bot: &Bot, chat_id: ChatId) -> HandlerResult {
        bot.send_message(chat_id, "Сколько тебе лет?".to_string())
            .await?;
        Ok(())
    }

    pub async fn dispatcher() {
        let bot = Bot::from_env();
        let handler = dptree::entry()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(Update::filter_message().endpoint(SwagaBot::message_handler))
            .branch(Update::filter_callback_query().endpoint(SwagaBot::callback_handler))
            .branch(Update::filter_inline_query().endpoint(SwagaBot::inline_query_handler));

        Dispatcher::builder(bot, handler)
            
            .dependencies(dptree::deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }

    async fn list_options(bot: &Bot, username: &str, chat_id: ChatId) -> HandlerResult {
        let keyboard = vec![vec![
            KeyboardButton::new("1"),
            KeyboardButton::new("2"),
            KeyboardButton::new("3"),
            KeyboardButton::new("4"),
        ]];
        let keyboard_markup = KeyboardMarkup::new(keyboard)
            .persistent()
            .resize_keyboard()
            .selective()
            .one_time_keyboard();
        bot.send_message(chat_id, "Так выглядит твоя анкета")
            .await?;
        bot.send_message(chat_id, format!("Username {username}"))
            .await?;
        bot.send_message(chat_id, "1.Смотреть анкеты\n2.Заполнить анкету заново\n3.Изменить фото/видео\n4.Изменить текст анкеты").reply_markup(keyboard_markup).await?;
        Ok(())
    }

    async fn start(bot: Bot, msg: Message) -> HandlerResult {
        if let Some(from) = msg.from {
            let profile = Profile::new(from.id.0 as i64, from.username);
            let matched_profile = match Profile::get_by_username(profile.username())? {
                Some(profile) => profile,
                None => profile.insert()?,
            };
            let chat_id = ChatId(*matched_profile.user_id());
            Self::send_welcome_message(&bot, matched_profile.username(), chat_id).await?;
            Self::list_options(&bot, matched_profile.username(), chat_id).await?;
        } else {
            panic!("todo: Msg from must be there");
        }
        Ok(())
    }

    async fn send_welcome_message(bot: &Bot, username: &str, chat_id: ChatId) -> HandlerResult {
        let db_profile = Profile::get_by_username(username)?;
        match db_profile {
            Some(profile) => {
                bot.send_message(
                    chat_id,
                    format!(
                        "Your user_id={}, username={}",
                        profile.user_id(),
                        profile.username()
                    ),
                )
                .await?;
            }
            None => unreachable!(),
        }
        Ok(())
    }
    async fn inline_query_handler(bot: Bot, q: InlineQuery) -> HandlerResult {
        let choose_debian_version = InlineQueryResultArticle::new(
            "0",
            "Chose debian version",
            InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
        );

        bot.answer_inline_query(q.id, vec![choose_debian_version.into()])
            .await?;

        Ok(())
    }

    /// When it receives a callback from a button it edits the message with all
    /// those buttons writing a text with the selected Debian version.
    ///
    /// **IMPORTANT**: do not send privacy-sensitive data this way!!!
    /// Anyone can read data stored in the callback button.
    async fn callback_handler(bot: Bot, q: CallbackQuery) -> HandlerResult {
        if let Some(ref version) = q.data {
            let text = format!("You chose: {version}");

            // Tell telegram that we've seen this query, to remove 🕑 icons from the
            // clients. You could also use `answer_callback_query`'s optional
            // parameters to tweak what happens on the client side.
            bot.answer_callback_query(&q.id).await?;

            // Edit text of the message to which the buttons were attached
            if let Some(message) = q.regular_message() {
                bot.edit_text(message, text).await?;
            } else if let Some(id) = q.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }

            log::info!("You chose: {}", version);
        }

        Ok(())
    }
}
