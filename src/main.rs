use dotenvy::dotenv;
use myself::{agent::Agent, database::MemoryEngine};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent = Agent::new_with_defaults(
        "Bregy".to_string(),
        "A simple communicative chatbot".to_string(),
    )
    .await;

    agent.interact("Hello World".to_string()).await;

    // database_core.new_interaction("bregy_2".to_string()).await;
}
