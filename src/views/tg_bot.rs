use teloxide::prelude::*;

use super::view_trait::View;


pub struct TelegramBotView {
    bot_token: String,
    bot: AutoSend<Bot>,
    process_chats: Vec<ChatId>
}

impl TelegramBotView {
    pub fn new(token: &str) -> Self {
        Self {
            bot_token: token.to_owned(),
            bot: Bot::new(token).auto_send(),
            process_chats: Vec::new(),
        }
    }

    pub fn connect_chat(&mut self, chat_id: i64) {
        self.process_chats.push(ChatId(chat_id));
    }
}



impl View for TelegramBotView {
    async fn show(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        for chat_id in &self.process_chats {
            self.bot.send_message(
                *chat_id, message
            ).await?;
        }

        Ok(())
    }
}
