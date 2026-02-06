use crate::router_agent::SharedRouterAgent;
use teloxide::prelude::*;

pub async fn start(agent: SharedRouterAgent) {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();
    let agent_for_handler = agent.clone();
    std::env::var("OLLAMA_API_BASE_URL").expect("OLLAMA_API_BASE_URL not set in .env file");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = agent_for_handler.clone();

        async move {
            // Use message text as prompt (fallback to a default)

            if let Some(user) = &msg.from {
                let user_id = user.id;

                let allowed_user_id: i64 = std::env::var("MY_TELEGRAM_USER_ID")
                    .expect("MY_TELEGRAM_USER_ID not set in .env file")
                    .parse()
                    .expect("MY_TELEGRAM_USER_ID must be a valid integer");
                
                if user_id.0 != allowed_user_id as u64 {
                    bot.send_message(msg.chat.id, "Unauthorized user").await?;
                    return Ok(());
                }
            } else {
                bot.send_message(msg.chat.id, "Could not identify user").await?;
                return Ok(());
            }

            let input = msg.text().unwrap_or("hello").to_string();
            let guard = agent.lock().await;
            if let Some(router) = guard.as_ref() {
                match router.prompt(&input).await {
                    Ok(reply) => {
                        bot.send_message(msg.chat.id, reply )
                            .await?;
                    }
                    Err(err) => {
                        bot.send_message(msg.chat.id, format!("error: {}", err))
                            .await?;
                    }
                }
            }
            Ok(())
        }
    })
    .await;
}
