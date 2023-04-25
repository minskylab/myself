use std::marker::PhantomData;

use crate::{agent::Agent, backend::AgentBackend, database::memory::MemoryEngine};

pub struct AgentBuilder<Backend>
where
    Backend: AgentBackend + Sized + Default,
{
    agent_name: String,
    default_user_name: String,
    default_constitution: String,
    default_memory_size: usize,
    database_url: String,
    backend: PhantomData<Backend>,
}
impl<Backend> Default for AgentBuilder<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Backend> AgentBuilder<Backend>
where
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub fn new() -> AgentBuilder<Backend> {
        AgentBuilder::<Backend> {
            agent_name: std::env::var("AGENT_NAME").unwrap_or("Agent".to_string()),
            default_user_name: std::env::var("DEFAULT_USER_NAME").unwrap_or("User".to_string()),
            default_constitution: std::env::var("DEFAULT_CONSTITUTION")
                .unwrap_or("A simple communicative chatbot".to_string()),
            default_memory_size: std::env::var("DEFAULT_MEMORY_SIZE")
                .unwrap_or("10".to_string())
                .parse()
                .unwrap_or(10),
            database_url: std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()),
            backend: PhantomData,
        }
    }

    pub fn name(&mut self, my_name: String) -> &mut Self {
        self.agent_name = my_name;
        self
    }

    pub fn default_constitution(&mut self, constitution: String) -> &mut Self {
        self.default_constitution = constitution;
        self
    }

    pub fn default_user_name(&mut self, user_name: String) -> &mut Self {
        self.default_user_name = user_name;
        self
    }

    pub fn default_memory_size(&mut self, memory_size: usize) -> &mut Self {
        self.default_memory_size = memory_size;
        self
    }

    pub fn database_url(&mut self, database_url: String) -> &mut Self {
        self.database_url = database_url;
        self
    }

    pub async fn build(&mut self, llm_engine: Backend) -> Agent<Backend> {
        let mut memory_engine = MemoryEngine::new(self.database_url.to_owned()).await;

        memory_engine
            .new_agent(
                self.agent_name.to_owned(),
                self.default_user_name.to_owned(),
                self.default_constitution.to_owned(),
                self.default_memory_size,
                llm_engine,
                memory_engine.clone(),
            )
            .await
    }
}
