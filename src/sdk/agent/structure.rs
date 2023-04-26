use crate::backend::core::AgentBackend;
use crate::database::memory::MemoryEngine;

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
    pub fn get_memory_engine(&self) -> Option<Box<MemoryEngine<Backend>>> {
        self.memory_engine.clone()
    }

    pub fn get_backend(&self) -> Option<Box<Backend>> {
        self.backend.clone()
    }
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
}
