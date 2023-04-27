use dotenvy::dotenv;
use myself::sdk::agent::AgentBuilder;

#[tokio::main]
async fn main() {
    // Don't forget to create a .env file with the following content:
    // OPENAI_API_KEY=your_api_key

    dotenv().ok();

    let mut agent = AgentBuilder::new()
        .name("AI".to_string())
        .build_default()
        .await;

    let message = "Hello World".to_string();
    let (_, output) = agent.interact_default(&message).await.unwrap();

    println!("{}", output.content);
}
