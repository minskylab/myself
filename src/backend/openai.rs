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
        input: InteractionBlock,
    ) -> InteractionBlock {
        let compiled_interaction_blocks = interaction
            .long_term_memory(50)
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

        let prompt = format!(
            "{}\n{}\n{}: {}\n{}: ",
            compiled_interaction_blocks,
            interaction.short_term_memory.clone(),
            interaction.user_name,
            input.content,
            input.name.unwrap_or(input.role.to_string()),
        );

        println!("Prompt:\n=======\n{}\n=======", prompt);

        let response = self.engine.completions_call(prompt, None).await.unwrap();

        let model_response = response.choices[0].text.trim().to_string();

        // database_core
        //     .append_to_long_term_memory(
        //         interaction.id,
        //         interaction.user_name,
        //         message.to_owned(),
        //         self.my_name.to_owned(),
        //         model_response.to_string(),
        //     )
        //     .await;

        // todo!()

        InteractionBlock::new(
            input.role,
            model_response,
            Some(interaction.agent.unwrap().my_name),
            interaction.id,
        )
    }
}
