use dotenvy::dotenv;
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent = AgentBuilder::new()
        .with_name("AI".to_string())
        .build()
        .await;

    let message = "Hello World".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{}", response);
}
