use crate::backend::core::AgentBackend;
use crate::database::memory::MemoryEngine;
use crate::sdk::interactions::Interaction;
use crate::sdk::interactions::InteractionBlock;
use crate::sdk::interactions::InteractionBlockRole;
use crate::sdk::interactions::WithAgent;
use crate::sdk::interactions::WithoutAgent;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DefaultInteraction {
    pub user_name: String,
    pub constitution: String,
    pub memory_size: usize,
}

#[derive(Clone, Debug)]
pub struct Agent<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub id: Uuid,
    pub my_name: String,
    pub default_interaction: DefaultInteraction,

    backend: Option<Box<Backend>>,
    memory_engine: Option<Box<MemoryEngine<Backend>>>,
}

impl<Backend> Agent<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub fn new(
        id: Uuid,
        my_name: String,
        default_interaction: DefaultInteraction,
        llm_engine: Backend,
        memory_engine: MemoryEngine<Backend>,
    ) -> Self {
        Self {
            id,
            my_name,
            default_interaction,
            backend: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    pub async fn forgot_short_term_memory(
        &mut self,
        interaction_id: Uuid,
    ) -> Option<Interaction<Backend>> {
        let interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;

        match interaction {
            Some(interaction) => {
                let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();

                Some(
                    database_core
                        .set_short_term_memory(interaction.id, "".to_string())
                        .await,
                )
            }
            None => None,
        }
    }

    pub async fn interact(&mut self, interaction_id: Uuid, message: &String) -> Option<String> {
        let interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;

        match interaction {
            Some(interaction) => {
                let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();

                let res = self
                    .backend
                    .clone()
                    .unwrap()
                    .predict_response(
                        interaction.clone().with_agent(self.clone()),
                        InteractionBlock::new(
                            InteractionBlockRole::Agent,
                            message.to_owned(),
                            Some(self.my_name.to_owned()),
                            interaction_id,
                        ),
                    )
                    .await;

                database_core
                    .append_to_long_term_memory(
                        interaction.id,
                        interaction.user_name,
                        message.to_owned(),
                        self.my_name.to_owned(),
                        res.content.to_owned(),
                    )
                    .await;

                Some(res.content)
            }
            None => None,
        }
    }

    pub async fn interact_default(&mut self, message: &String) -> Option<String> {
        self.clone()
            .memory_engine
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
        self.clone()
            .memory_engine
            .as_mut()
            .unwrap()
            .new_interaction_with_agent(user_name, constitution, memory_size, self)
            .await
    }

    pub async fn init_interaction_defaults(
        &mut self,
        new_user_name: Option<String>,
    ) -> Interaction<Backend, WithoutAgent> {
        self.memory_engine
            .as_mut()
            .unwrap()
            .new_interaction(
                new_user_name.unwrap_or(self.default_interaction.user_name.clone()),
                self.default_interaction.constitution.clone(),
                self.default_interaction.memory_size,
            )
            .await
    }

    pub async fn get_interaction(
        &mut self,
        interaction_id: Uuid,
    ) -> Option<Interaction<Backend, WithoutAgent>> {
        self.memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction<Backend, WithoutAgent>> {
        self.memory_engine
            .as_mut()
            .unwrap()
            .get_all_interactions()
            .await
    }

    pub async fn get_default_interaction(&mut self) -> Interaction<Backend, WithAgent> {
        self.clone()
            .memory_engine
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
        self.memory_engine
            .as_mut()
            .unwrap()
            .update_constitution(interaction_id, constitution)
            .await
    }

    pub fn memory_engine(&mut self) -> &mut Box<MemoryEngine<Backend>> {
        self.memory_engine.as_mut().unwrap()
    }
}
