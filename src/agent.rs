use crate::{
    database::{Interaction, MemoryEngine},
    llm::LLMEngine,
};
use rbdc::uuid::Uuid;

#[derive(Clone)]
pub struct DefaultInteraction {
    pub user_name: String,
    pub constitution: String,
    pub memory_size: usize,
}

#[derive(Clone)]
pub struct Agent {
    pub my_name: String,
    pub default_interaction: DefaultInteraction,

    llm_engine: Option<Box<LLMEngine>>,
    memory_engine: Option<Box<MemoryEngine>>,
}

trait LLMAgent {
    fn interact(&mut self, message: String) -> String;
}

impl Agent {
    pub fn new(
        my_name: String,
        default_interaction: DefaultInteraction,
        llm_engine: LLMEngine,
        memory_engine: MemoryEngine,
    ) -> Self {
        Self {
            my_name,
            default_interaction,
            llm_engine: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    pub async fn forget_short_term_memory(&mut self, interaction_id: Uuid) -> Option<Interaction> {
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
                        .set_short_term_memory(interaction.id.clone(), "".to_string())
                        .await,
                )
            }
            None => None,
        }
    }

    pub async fn interact_with(
        &mut self,
        interaction_id: Uuid,
        message: &String,
    ) -> Option<String> {
        let interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;

        match interaction {
            Some(interaction) => {
                let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();
                let llm_engine = self.llm_engine.as_mut().unwrap().to_owned();

                let prompt = format!(
                    "{}\n{}\n{}: {}\n{}: ",
                    interaction.long_term_memory,
                    interaction
                        .short_term_memory
                        .clone()
                        .unwrap_or("".to_string()),
                    interaction.user_name,
                    message,
                    self.my_name,
                );

                let response = llm_engine.completions_call(prompt, None).await.unwrap();

                let model_response = response.choices[0].text.trim();

                database_core
                    .append_to_dynamic_memory(
                        interaction.id,
                        format!(
                            "{}: {}\n{}: {}",
                            interaction.user_name, message, self.my_name, model_response
                        ),
                    )
                    .await;

                Some(model_response.into())
            }
            None => None,
        }
    }

    pub async fn interact_default(&mut self, message: &String) -> Option<String> {
        let default_interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_or_create_default_interaction(
                self.default_interaction.user_name.clone(),
                self.default_interaction.constitution.clone(),
                self.default_interaction.memory_size,
            )
            .await;

        self.interact_with(default_interaction.id, message).await
    }

    pub async fn init_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        self.memory_engine
            .as_mut()
            .unwrap()
            .new_interaction(user_name, constitution, memory_size)
            .await
    }

    pub async fn init_interaction_defaults(
        &mut self,
        new_user_name: Option<String>,
    ) -> Interaction {
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

    pub async fn get_interaction(&mut self, interaction_id: Uuid) -> Option<Interaction> {
        self.memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction> {
        self.memory_engine
            .as_mut()
            .unwrap()
            .get_all_interactions()
            .await
    }

    pub async fn get_default_interaction(&mut self) -> Interaction {
        self.memory_engine
            .as_mut()
            .unwrap()
            .get_or_create_default_interaction(
                self.default_interaction.user_name.clone(),
                self.default_interaction.constitution.clone(),
                self.default_interaction.memory_size,
            )
            .await
    }

    pub async fn update_long_term_memory(
        &mut self,
        interaction_id: Uuid,
        constitution: String,
    ) -> Interaction {
        self.memory_engine
            .as_mut()
            .unwrap()
            .update_constitution(interaction_id, constitution)
            .await
    }
}
