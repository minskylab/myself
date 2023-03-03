// use rbdc::{datetime::FastDateTime, uuid::Uuid};

use dotenvy::dotenv;
use rbdc::uuid::Uuid;
// use serde::{Deserialize, Serialize};

use crate::{
    database::{Interaction, MemoryEngine},
    llm::LLMEngine,
};

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

    pub async fn new_with_defaults(my_name: String) -> Self {
        dotenv().ok();

        let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let default_user_name = std::env::var("DEFAULT_USER_NAME").unwrap_or("User".to_string());
        let default_constitution = std::env::var("DEFAULT_CONSTITUTION")
            .unwrap_or("A simple communicative chatbot".to_string());
        let default_memory_size = std::env::var("DEFAULT_MEMORY_SIZE")
            .unwrap_or("10".to_string())
            .parse()
            .unwrap_or(10);

        let sql_url =
            std::env::var("SQLITE_URL").unwrap_or("sqlite://target/sqlite.db".to_string());

        let llm_engine = LLMEngine::new(openai_api_key);

        let memory_engine = MemoryEngine::new(sql_url).await;

        Self {
            my_name,
            default_interaction: DefaultInteraction {
                user_name: default_user_name,
                constitution: default_constitution,
                memory_size: default_memory_size,
            },
            llm_engine: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    pub async fn forget(&mut self, interaction_id: Uuid) -> Interaction {
        let interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;
        let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();

        database_core
            .set_dynamic_memory(interaction.id.clone(), "".to_string())
            .await
    }

    pub async fn interact_with(&mut self, interaction_id: Uuid, message: String) -> String {
        let interaction = self
            .memory_engine
            .as_mut()
            .unwrap()
            .get_interaction(interaction_id)
            .await;
        let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();
        let llm_engine = self.llm_engine.as_mut().unwrap().to_owned();

        let prompt = format!(
            "{}\n{}\n{}: {}\n{}: ",
            interaction.template_memory,
            interaction.dynamic_memory.clone().unwrap_or("".to_string()),
            interaction.user_name,
            message,
            self.my_name,
        );

        // dbg!(prompt.clone());

        let response = llm_engine.completions_call(prompt, None).await.unwrap();

        let model_response = response.choices[0].text.trim();

        // dbg!(model_response.clone());

        database_core
            .append_to_dynamic_memory(
                interaction.id,
                format!(
                    "{}: {}\n{}: {}",
                    interaction.user_name, message, self.my_name, model_response
                ),
            )
            .await;

        model_response.into()
    }

    pub async fn interact_with_default(&mut self, message: String) -> String {
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

    pub async fn get_interaction(&mut self, interaction_id: Uuid) -> Interaction {
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
}
