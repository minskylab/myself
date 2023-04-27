use uuid::Uuid;

use crate::backend::core::AgentBackend;
use crate::database::memory::MemoryEngine;

use crate::sdk::interaction::Interaction;
use crate::sdk::interaction::InteractionBlock;
use crate::sdk::interaction::WithAgent;
use crate::sdk::interaction::WithoutAgent;

use super::structure::Agent;

impl<Backend> Agent<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub async fn forgot_short_term_memory(
        &mut self,
        interaction_id: Uuid,
    ) -> Option<Interaction<Backend>> {
        let interaction = self
            .get_memory_engine()
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;

        match interaction {
            Some(interaction) => {
                let mut database_core = self.get_memory_engine().as_mut().unwrap().to_owned();

                Some(
                    database_core
                        .set_short_term_memory(interaction.id, "".to_string())
                        .await,
                )
            }
            None => None,
        }
    }

    pub async fn interact(
        &mut self,
        interaction_id: Uuid,
        message: &String,
    ) -> Option<(InteractionBlock, InteractionBlock)> {
        let interaction = self
            .get_memory_engine()
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;

        let interaction_in = InteractionBlock::new_agent(
            interaction_id,
            message.to_owned(),
            Some(self.my_name.to_owned()),
        );

        match interaction {
            Some(interaction) => {
                let mut memory_engine = self.get_memory_engine().as_mut().unwrap().to_owned();

                let interaction_out = self
                    .get_backend()
                    .clone()
                    .unwrap()
                    .predict_response(
                        interaction.clone().with_agent(self.clone()),
                        &interaction_in,
                    )
                    .await;

                let interaction_in = memory_engine
                    .append_to_long_term_memory(interaction_id, &interaction_in)
                    .await;

                let interaction_out = memory_engine
                    .append_to_long_term_memory(interaction_id, &interaction_out)
                    .await;

                Some((interaction_in, interaction_out))
            }
            None => None,
        }
    }

    pub async fn interact_default(
        &mut self,
        message: &String,
    ) -> Option<(InteractionBlock, InteractionBlock)> {
        self.clone()
            .get_memory_engine()
            .as_mut()
            .unwrap()
            .get_or_create_default_interaction(self)
            .await
            .interact(message)
            .await
    }

    pub async fn init_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction<Backend, WithAgent> {
        self.get_memory_engine()
            .clone()
            .as_mut()
            .unwrap()
            .new_interaction_with_agent(user_name, constitution, memory_size, self)
            .await
    }

    pub async fn init_interaction_defaults(
        &mut self,
        new_user_name: Option<String>,
    ) -> Interaction<Backend, WithAgent> {
        self.get_memory_engine()
            .clone()
            .as_mut()
            .unwrap()
            .new_interaction_with_agent(
                new_user_name.unwrap_or(self.default_interaction.user_name.clone()),
                self.default_interaction.constitution.clone(),
                self.default_interaction.memory_size,
                self,
            )
            .await
    }

    pub async fn get_interaction(
        &mut self,
        interaction_id: Uuid,
    ) -> Option<Interaction<Backend, WithoutAgent>> {
        self.get_memory_engine()
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction<Backend, WithoutAgent>> {
        self.get_memory_engine()
            .as_mut()
            .unwrap()
            .get_all_interactions()
            .await
    }

    pub async fn get_default_interaction(&mut self) -> Interaction<Backend, WithAgent> {
        self.clone()
            .get_memory_engine()
            .as_mut()
            .unwrap()
            .get_or_create_default_interaction(self)
            .await
    }

    pub async fn update_long_term_memory(
        &mut self,
        interaction_id: Uuid,
        constitution: String,
    ) -> Interaction<Backend> {
        self.get_memory_engine()
            .as_mut()
            .unwrap()
            .update_constitution(interaction_id, constitution)
            .await
    }

    pub fn memory_engine(&mut self) -> Box<MemoryEngine<Backend>> {
        self.get_memory_engine().unwrap()
    }
}
