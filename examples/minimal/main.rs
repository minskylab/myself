use dotenvy::dotenv;
use myself::{agent_builder::AgentBuilder, backend::OpenAIBackend};
use std::env::var;

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let llm_engine = OpenAIBackend::new(var("OPENAI_API_KEY").unwrap());

    let mut agent = AgentBuilder::new()
        .name("AI".to_string())
        .build(llm_engine)
        .await;

    let message = "Hello World".to_string();
    let response = agent.interact_default(&message).await.unwrap();

    println!("{}", response);
}
