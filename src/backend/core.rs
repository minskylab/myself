use crate::sdk::interaction::{Interaction, InteractionBlock, WithAgent};
use async_trait::async_trait;

#[async_trait]
pub trait AgentBackend
where
    Self: Sized + Default + Clone,
{
    async fn predict_response(
        &mut self,
        interaction: Interaction<Self, WithAgent>,
        input: &InteractionBlock,
    ) -> InteractionBlock;
}
