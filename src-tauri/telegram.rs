use crate::{
    adme::{Adme},
    filters::filter_think_tag,
};
use teloxide::prelude::*;

pub async fn start(agent: Adme) {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = agent.clone();

        async move {
            // Use message text as prompt (fallback to a default)
            if validate_telegram_user_id(&msg) == false {
                bot.send_message(msg.chat.id, "Unauthorized user").await?;
                Ok(())
            } else {
                let input = msg.text().unwrap_or("hello").to_string();

                if input.eq_ignore_ascii_case("/start") {
                    bot.send_message(msg.chat.id, "Welcome to the bot!").await?;
                    return Ok(());
                }

                let mut reply = agent.prompt(&input).await;
                reply = filter_think_tag(&reply);
                bot.send_message(msg.chat.id, &reply).await?;

                Ok(())
            }
        }
    })
    .await;
}

fn validate_telegram_user_id(msg: &Message) -> bool {
    if let Some(user) = &msg.from {
        let user_id = user.id;

        let allowed_user_id: i64 = std::env::var("MY_TELEGRAM_USER_ID")
            .expect("MY_TELEGRAM_USER_ID not set in .env file")
            .parse()
            .expect("MY_TELEGRAM_USER_ID must be a valid integer");

        if user_id.0 != allowed_user_id as u64 {
            return false;
        }
        true
    } else {
        false
    }
}
