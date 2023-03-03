use dotenvy::dotenv;
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let mut agent = AgentBuilder::new()
        .with_name("AI".to_string())
        .build()
        .await;

    let message = "Hello World".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{}", response);
}
