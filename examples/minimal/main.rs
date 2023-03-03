use myself::agent::Agent;

#[tokio::main]
async fn main() {
    let mut agent = Agent::new_with_defaults("AI".to_string()).await;

    agent.interact_with_default("Hello World".to_string()).await;
}
