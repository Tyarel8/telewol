use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{utils, wol};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "awake the computer. Usage: `/wake <computer|MAC>`")]
    Wake(String),
    #[command(description = "list all computers.")]
    List,
    #[command(
        description = "add a computer with its MAC address. Usage: `/add <computer> <MAC>`",
        parse_with = "split"
    )]
    Add { computer: String, mac: String },
    #[command(description = "remove computer. Usage: `/remove <computer>`")]
    Remove { computer: String },
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    if matches!(msg.chat.kind, teloxide::types::ChatKind::Private(_)) {
        bot.send_message(msg.chat.id, "@TeleWolBot is only available in groups")
            .await?;
        return Ok(());
    }

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::List => {
            let config = match utils::Config::load() {
                Ok(config) => config,
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                    return Ok(());
                }
            };
            bot.send_message(msg.chat.id, config.list_computers())
                .await?;
        }
        Command::Add { computer, mac } => {
            let new_mac = match wol::parse_mac(&mac) {
                Ok(mac) => mac,
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                    return Ok(());
                }
            };

            let mut config = match utils::Config::load() {
                Ok(config) => config,
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                    return Ok(());
                }
            };

            config
                .computers
                .insert(computer.clone(), wol::unparse_mac(new_mac));

            match config.save() {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("`{}` added successfully", computer))
                        .await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                }
            }
        }
        Command::Remove { computer } => {
            let mut config = match utils::Config::load() {
                Ok(config) => config,
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                    return Ok(());
                }
            };

            if !config.computers.contains_key(&computer) {
                bot.send_message(msg.chat.id, format!("`{}` not found", computer))
                    .await?;
                return Ok(());
            } else {
                config.computers.remove(&computer);
            }

            match config.save() {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("`{}` removed successfully", computer))
                        .await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, err.to_string()).await?;
                }
            }
        }
        Command::Wake(computer) => {
            if wol::parse_mac(&computer).is_ok() {
                match wol::wake(&computer) {
                    Ok(_) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("`{}` has awoken from its slumber", computer),
                        )
                        .await?;
                    }
                    Err(err) => {
                        bot.send_message(msg.chat.id, err.to_string()).await?;
                    }
                }
            } else {
                let config = match utils::Config::load() {
                    Ok(config) => config,
                    Err(err) => {
                        bot.send_message(msg.chat.id, err.to_string()).await?;
                        return Ok(());
                    }
                };

                let mac = match config.computers.get(&computer) {
                    Some(mac) => mac,
                    None => {
                        bot.send_message(msg.chat.id, format!("`{}` not found", computer))
                            .await?;
                        return Ok(());
                    }
                };

                match wol::wake(mac) {
                    Ok(_) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("`{}` has awoken from its slumber", computer),
                        )
                        .await?;
                    }
                    Err(err) => {
                        bot.send_message(msg.chat.id, err.to_string()).await?;
                    }
                }
            }
        }
    };

    Ok(())
}
