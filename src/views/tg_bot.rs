use std::collections::HashSet;
use teloxide::{
    *, prelude::*,
    types::ParseMode, utils::html,
    dispatching::DefaultKey,
    utils::command::BotCommands
};
use tokio::task;
use std::sync::Arc;
use tokio::sync::Mutex; // for async access to the data

use super::view_trait::View;



#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Hello, I'm bot which will notify you about air alerts in your regions.\nList of supported commands:")]
enum Commands {
    #[command(description = "get a help message.")]
    Help,
    #[command(description = "start tracking")]
    Start,
    #[command(description = "stop tracking")]
    Stop,
    #[command(description = "check if bot is running")]
    Check,
}



// handlers for the Commands
async fn handle_start(bot: Bot, msg: Message, cmd: Commands, chat_state: Arc<Mutex<HashSet<ChatId>>>)
    -> ResponseResult<()> {
    let mut chats = chat_state.lock().await;
    
    chats.insert(msg.chat.id);
    bot.send_message(
        msg.chat.id, "The tracker started.".to_owned()
    ).await?;

    Ok(())
}
    
async fn handle_stop(bot: Bot, msg: Message, _: Commands, chat_state: Arc<Mutex<HashSet<ChatId>>>)
    -> ResponseResult<()> {
    let mut chats = chat_state.lock().await;
    
    if chats.contains(&msg.chat.id) {
        chats.remove(&msg.chat.id);
        bot.send_message(
            msg.chat.id, "The tracker stopped."
        ).await?;
    } else {
        bot.send_message(
            msg.chat.id, "The tracker has not been started."
        ).await?;
    }

    Ok(())
}

async fn handle_help(bot: Bot, msg: Message, _: Commands, _: Arc<Mutex<HashSet<ChatId>>>)
    -> ResponseResult<()> {
    bot.send_message(
            msg.chat.id, Commands::descriptions().to_string()
        ).await?;
    
    Ok(())
}


async fn handle_check(bot: Bot, msg: Message, cmd: Commands, _: Arc<Mutex<HashSet<ChatId>>>)
    -> ResponseResult<()> {
    bot.send_message(
            msg.chat.id, "Bot is running."
        ).await?;

    Ok(())
}



pub struct TelegramBotView {
    bot: Bot,
    dispatcher: Arc<Mutex<Dispatcher<Bot, RequestError, DefaultKey>>>,
    process_chats: Arc<Mutex<HashSet<ChatId>>>
}

impl TelegramBotView {
    pub async fn new(token: &str) -> Self {
        let handler = Update::filter_message()
            .filter_command::<Commands>()  // Parse as Commands enum
            .branch(
                dptree::case![Commands::Start]  // Match specific command
                    .endpoint(handle_start)
            )
            .branch(
                dptree::case![Commands::Stop]
                    .endpoint(handle_stop)
            ).branch(
                dptree::case![Commands::Check]
                    .endpoint(handle_check)
            ).branch(
                dptree::case![Commands::Help]
                    .endpoint(handle_help)
            );


        let chat_state = Arc::new(
                Mutex::new(HashSet::new())
            );

        let bot = Bot::new(token);
        let dispatcher = Arc::new(Mutex::new(
            Dispatcher::builder(bot.clone(), handler)
            .dependencies(dptree::deps![chat_state.clone()])
            .enable_ctrlc_handler()
            .build()
        ));

        Self {
            bot: bot,
            dispatcher: dispatcher,
            process_chats: chat_state
        }
    }


    pub async fn connect_chat(&mut self, chat_id: i64) {
        let mut chats = self.process_chats.lock().await;
        chats.insert(ChatId(chat_id));
    }


    pub async fn start_bot(&mut self) {
        let mut dispatcher = Arc::clone(
            &self.dispatcher// make a clone of a pointer
        );

        task::spawn(async move {
            let mut dispatcher_clone = dispatcher.lock().await;
            dispatcher_clone.dispatch().await;
        });
    }
}



impl View for TelegramBotView {
    async fn show(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut chats = self.process_chats.lock().await;

        for chat_id in chats.iter() {
            self.bot.send_message(
                *chat_id, message
            ).parse_mode(ParseMode::Html).await?;
        }

        Ok(())
    }
}



