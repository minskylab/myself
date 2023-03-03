use myself::agent::Agent;

#[tokio::main]
async fn main() {
    let mut agent = Agent::new_with_defaults("AI".to_string()).await;

    let message = "Hello World".to_string();
    let response = agent.interact_with_default(&message).await.unwrap();

    println!("{} -> {}", message, response);
}
