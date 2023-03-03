use dotenvy::dotenv;
use myself::agent_builder::AgentBuilder;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent = AgentBuilder::new()
        .with_name("AI (Agent)".to_string())
        .build()
        .await;

    let interaction = agent
        .init_interaction(
            "Joe (Human)".to_string(),
            "A talkative chatbot conversation".to_string(),
            40,
        )
        .await;

    let message = "How are you?, explain please".to_string();
    let response = agent.interact_with(interaction.id, &message).await.unwrap();

    println!("{}", response);
}
