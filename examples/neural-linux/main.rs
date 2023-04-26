use std::io::{stdin, Write};

use dotenvy::dotenv;
use myself::{
    backend::openai::OpenAIBackend, database::memory::MemoryEngine, sdk::agent::AgentBuilder,
};
use std::env::var;

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let llm_engine = OpenAIBackend::new(var("OPENAI_API_KEY").unwrap());
    let memory_engine = MemoryEngine::new(var("DATABASE_URL").unwrap()).await;

    let mut agent = AgentBuilder::new()
        .name("Linux Server".to_string())
        .default_user_name("User".to_string())
        .default_constitution("I want you to act as a linux terminal. I will type commands and you will reply with what the terminal should show. I want you to only reply with the terminal output inside one unique code block, and nothing else. do not write explanations. do not type commands unless I instruct you to do so. When I need to tell you something in English, I will do so by putting text inside curly brackets {like this}.".into())
        .default_memory_size(50)
        .build(llm_engine, memory_engine)
        .await;

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        let (_, response) = agent.interact_default(&user_input).await.unwrap();

        println!("{}", response.content);
    }
}
