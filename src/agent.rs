// use rbdc::{datetime::FastDateTime, uuid::Uuid};

use crate::{
    database::{Interaction, MemoryEngine},
    llm::LLMEngine,
};

pub struct Agent {
    pub name: String,
    pub interaction: Interaction,

    pub llm_engine: Option<Box<LLMEngine>>,
    pub memory_engine: Option<Box<MemoryEngine>>,
}

impl Agent {
    pub fn new(
        name: String,
        interaction: Interaction,
        llm_engine: LLMEngine,
        memory_engine: MemoryEngine,
    ) -> Self {
        Self {
            name,
            interaction,
            llm_engine: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    pub async fn new_with_defaults(name: String, constitution: String) -> Self {
        let llm_engine = LLMEngine::new(std::env::var("OPENAI_API_KEY").unwrap());
        let mut memory_engine = MemoryEngine::new().await;

        let interaction = match memory_engine.get_default_interaction().await {
            Some(interaction) => {
                memory_engine
                    .update_constitution(interaction.id, constitution.clone())
                    .await
            }

            None => {
                memory_engine
                    .new_interaction(name.clone(), constitution)
                    .await
            }
        };

        Self {
            name,
            interaction,
            llm_engine: Some(Box::new(llm_engine)),
            memory_engine: Some(Box::new(memory_engine)),
        }
    }

    // pub fn set_llm_engine(&mut self, llm_engine: &'static LLMEngine) {
    //     self.llm_engine = Some(llm_engine);
    // }

    // pub fn set_database_core(&mut self, database_core: &'static MemoryEngine) {
    //     self.memory_engine = Some(database_core);
    // }

    // pub fn get_llm_engine(&self) -> Option<&'static LLMEngine> {
    //     self.llm_engine
    // }

    pub async fn interact(&mut self) -> String {
        let llm_engine = self.llm_engine.as_mut().unwrap().to_owned();
        let mut database_core = self.memory_engine.as_mut().unwrap().to_owned();

        let prompt = format!(
            "{}: {}\n{}: ",
            self.name, self.interaction.template_memory, self.name
        );

        let response = llm_engine.completions_call(prompt, None).await.unwrap();

        let model_response = response.choices[0].text.clone();

        self.interaction = database_core
            .append_to_dynamic_memory(
                self.interaction.id.clone(),
                format!("{}: {}", self.name, model_response),
            )
            .await;

        model_response
    }
}
