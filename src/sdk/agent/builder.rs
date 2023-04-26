use std::marker::PhantomData;

use crate::{
    backend::core::AgentBackend, database::memory::MemoryEngine, sdk::agent::structure::Agent,
};

pub struct AgentBuilder<Backend>
where
    Backend: AgentBackend + Sized + Default,
{
    agent_name: String,
    default_user_name: String,
    default_constitution: String,
    default_memory_size: usize,
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

    pub async fn build(
        &mut self,
        llm_engine: Backend,
        mut memory_engine: MemoryEngine<Backend>,
    ) -> Agent<Backend> {
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
