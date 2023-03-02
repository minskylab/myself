use crate::{
    database::{DatabaseCore, Interaction},
    llm::LLMEngine,
};

pub struct Agent {
    pub name: String,
    pub interaction: Interaction,

    pub llm_engine: Option<&'static LLMEngine>,
    pub database_core: Option<&'static DatabaseCore>,
}

impl Agent {
    pub fn new(
        name: String,
        interaction: Interaction,
        llm_engine: &'static LLMEngine,
        database_core: &'static DatabaseCore,
    ) -> Self {
        Self {
            name,
            interaction,
            llm_engine: Some(llm_engine),
            database_core: Some(database_core),
        }
    }

    pub fn set_llm_engine(&mut self, llm_engine: &'static LLMEngine) {
        self.llm_engine = Some(llm_engine);
    }

    pub fn set_database_core(&mut self, database_core: &'static DatabaseCore) {
        self.database_core = Some(database_core);
    }

    pub fn get_llm_engine(&self) -> Option<&'static LLMEngine> {
        self.llm_engine
    }

    pub async fn interact(&mut self) -> String {
        let llm_engine = self.get_llm_engine().unwrap().to_owned();
        let mut database_core = self.database_core.unwrap().to_owned();

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
