use myself::agent::Agent;

#[tokio::main]
async fn main() {
    let mut agent = Agent::new_with_defaults("Bob".to_string()).await;

    agent
        .interact_with_default("Give me an example of bubble sort written in python".to_string())
        .await;
}
