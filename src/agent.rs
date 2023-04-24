use crate::sdk::interactions::Interaction;
use crate::sdk::interactions::WithAgent;
use crate::{database::memory::MemoryEngine, llm::LLMEngine};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DefaultInteraction {
    pub user_name: String,
    pub constitution: String,
    pub memory_size: usize,
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: Uuid,
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
        id: Uuid,
        my_name: String,
        default_interaction: DefaultInteraction,
        llm_engine: LLMEngine,
        memory_engine: MemoryEngine,
    ) -> Self {
        Self {
            id,
            my_name,
            default_interaction,
            llm_engine: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    pub async fn forgot_short_term_memory(&mut self, interaction_id: Uuid) -> Option<Interaction> {
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
                let llm_engine = self.llm_engine.as_mut().unwrap().to_owned();

                let compiled_interaction_blocks = interaction
                    .long_term_memory(self, 50)
                    .await
                    .iter()
                    .map(|b| format!("{}: {}", b.role.as_str(), b.content))
                    .collect::<Vec<String>>()
                    .join("\n");

                let prompt = format!(
                    "{}\n{}\n{}: {}\n{}: ",
                    compiled_interaction_blocks,
                    interaction.short_term_memory.clone(),
                    interaction.user_name,
                    message,
                    self.my_name,
                );

                println!("Prompt: {}", prompt);

                let response = llm_engine.completions_call(prompt, None).await.unwrap();

                let model_response = response.choices[0].text.trim();

                database_core
                    .append_to_long_term_memory(
                        interaction.id,
                        interaction.user_name,
                        message.to_owned(),
                        self.my_name.to_owned(),
                        model_response.to_string(),
                    )
                    .await;

                Some(model_response.into())
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
    ) -> Interaction<WithAgent> {
        // self.memory_engine
        //     .as_mut()
        //     .unwrap()
        //     .new_interaction(user_name, constitution, memory_size)
        //     .await

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

    pub async fn get_default_interaction(&mut self) -> Interaction<WithAgent> {
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
    ) -> Interaction {
        self.memory_engine
            .as_mut()
            .unwrap()
            .update_constitution(interaction_id, constitution)
            .await
    }

    pub fn memory_engine(&mut self) -> &mut Box<MemoryEngine> {
        self.memory_engine.as_mut().unwrap()
    }
}
