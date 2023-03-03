use std::io::{stdin, Write};

use dotenvy::dotenv;
use myself::{
    agent::{Agent, DefaultInteraction},
    database::MemoryEngine,
    llm::LLMEngine,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent = Agent::new(
        "Linux Server".into(),
        DefaultInteraction {
            user_name: "User".into(),
            constitution: "I want you to act as a linux terminal. I will type commands and you will reply with what the terminal should show. I want you to only reply with the terminal output inside one unique code block, and nothing else. do not write explanations. do not type commands unless I instruct you to do so. When I need to tell you something in English, I will do so by putting text inside curly brackets {like this}.".into(),
            memory_size: 20,
        },
        LLMEngine::new_with_defaults(),
        MemoryEngine::new_with_defaults().await,
    );

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        println!(
            "{}",
            agent.interact_with_default(&user_input).await.unwrap()
        );
    }
}
