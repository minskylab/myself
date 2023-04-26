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
        .name("AI (Agent)".to_string())
        .build(llm_engine, memory_engine)
        .await;

    let mut joe_interaction = agent
        .init_interaction(
            "Joe (Human)".to_string(),
            "A talkative chatbot conversation".to_string(),
            10,
        )
        .await;

    let message = "How are you?, explain please".to_string();
    // let response = agent.interact(interaction.id, &message).await.unwrap();

    let (_, output) = joe_interaction.interact(&message).await.unwrap();

    println!("{}", output.content);
}
