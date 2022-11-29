// #![allow(unused)]

use teloxide::{dispatching::repls::CommandReplExt, Bot};

mod telegram;
mod utils;
mod wol;

#[tokio::main]
async fn main() {
    let config = match utils::Config::load() {
        Ok(config) => config,
        Err(err) => {
            panic!("{}", err);
        }
    };

    println!("Bot is running...");
    let bot = Bot::new(config.telegram_token);
    telegram::Command::repl(bot, telegram::answer).await;
}
