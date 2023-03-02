use myself::database::DatabaseCore;

#[tokio::main]
async fn main() {
    let mut database_core = DatabaseCore::new().await;

    let interaction = database_core
        .new_interaction("bregy_1".to_string(), "Hello, I am bregy_1".to_string())
        .await;

    println!("{:?}", interaction);

    database_core
        .update_constitution(
            interaction.id.clone(),
            "Hello, I am bregy_1, and I am updated".to_string(),
        )
        .await;

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 2".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 3".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 4".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 5".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 6".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 7".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 8".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 9".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 10".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id, "new_interaction 11".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 12".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id.clone(), "new_interaction 13".to_string())
        .await;

    println!("{:?}", interaction);

    let interaction = database_core
        .append_to_dynamic_memory(interaction.id, "new_interaction 14".to_string())
        .await;

    println!("{:?}", interaction);

    // database_core.new_interaction("bregy_2".to_string()).await;
}
