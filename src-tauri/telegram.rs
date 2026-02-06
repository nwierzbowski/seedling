use crate::router_agent::{SharedRouterAgent};
use teloxide::prelude::*;

pub async fn start(agent: SharedRouterAgent) {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();
    let agent_for_handler = agent.clone();

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let agent = agent_for_handler.clone();
        async move {
            // Use message text as prompt (fallback to a default)
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
    })
    .await;
}
