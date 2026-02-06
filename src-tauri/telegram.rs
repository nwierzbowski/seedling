use crate::router_agent::SharedRouterAgent;
use teloxide::prelude::*;

pub async fn start(agent: SharedRouterAgent) {
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
                let guard = agent.lock().await;
                if let Some(router) = guard.as_ref() {
                    match router.prompt(&input).await {
                        Ok(reply) => {
                            bot.send_message(msg.chat.id, reply).await?;
                        }
                        Err(err) => {
                            bot.send_message(msg.chat.id, format!("error: {}", err))
                                .await?;
                        }
                    }
                }
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
