use async_trait::async_trait;

use crate::{
    llm::LLMEngine,
    sdk::interactions::{Interaction, InteractionBlock, WithAgent},
};

use super::core::AgentBackend;

#[derive(Default, Clone)]
pub struct OpenAIBackend {
    pub engine: LLMEngine,
}

impl OpenAIBackend {
    pub fn new(api_key: String) -> Self {
        let engine = LLMEngine::new(api_key);
        Self { engine }
    }
}

#[async_trait]
impl AgentBackend for OpenAIBackend {
    async fn predict_response(
        &mut self,
        interaction: Interaction<Self, WithAgent>,
        input: &InteractionBlock,
    ) -> InteractionBlock {
        let compiled_interaction_blocks = interaction
            .long_term_memory(interaction.long_term_memory_size)
            .await
            .iter()
            .map(|b| {
                format!(
                    "{}: {}",
                    b.name.clone().unwrap_or(b.role.to_string()),
                    b.content
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let role_name = input.role.to_string();

        let prompt = format!(
            "{}\n{}\n{}: {}\n{}: ",
            compiled_interaction_blocks,
            interaction.short_term_memory.clone(),
            interaction.user_name,
            input.content,
            input.name.to_owned().unwrap_or(role_name),
        );

        println!("Prompt:\n=======\n{}\n=======", prompt);

        let response = self.engine.completions_call(prompt, None).await.unwrap();

        let model_response = response.choices[0].text.trim().to_string();

        InteractionBlock::new(
            input.role.clone(),
            model_response,
            interaction.id,
            Some(interaction.agent.unwrap().my_name),
        )
    }
}
