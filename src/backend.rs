use crate::sdk::interactions::{Interaction, InteractionBlock};
use async_trait::async_trait;

#[async_trait]
pub trait AgentBackend {
    async fn predict_response(
        &mut self,
        interaction: Interaction,
        input: InteractionBlock,
    ) -> InteractionBlock;
}

#[derive(Default, Clone)]
pub struct OpenAIBackend {
    pub api_key: String,
}

impl OpenAIBackend {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl AgentBackend for OpenAIBackend {
    async fn predict_response(
        &mut self,
        interaction: Interaction,
        input: InteractionBlock,
    ) -> InteractionBlock {
        todo!()
    }
}
