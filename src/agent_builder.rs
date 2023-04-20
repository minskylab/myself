use crate::{
    agent::{Agent, DefaultInteraction},
    database::MemoryEngine,
    llm::LLMEngine,
};

pub struct AgentBuilder {
    agent_name: String,
    openai_api_key: String,
    default_user_name: String,
    default_constitution: String,
    default_memory_size: usize,
    database_url: String,
}
impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentBuilder {
    pub fn new() -> AgentBuilder {
        AgentBuilder {
            openai_api_key: std::env::var("OPENAI_API_KEY").unwrap(),
            agent_name: std::env::var("AGENT_NAME").unwrap_or("AI".to_string()),
            default_user_name: std::env::var("DEFAULT_USER_NAME").unwrap_or("User".to_string()),
            default_constitution: std::env::var("DEFAULT_CONSTITUTION")
                .unwrap_or("A simple communicative chatbot".to_string()),
            default_memory_size: std::env::var("DEFAULT_MEMORY_SIZE")
                .unwrap_or("10".to_string())
                .parse()
                .unwrap_or(10),
            database_url: std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string()),
        }
    }

    pub fn name(&mut self, my_name: String) -> &mut Self {
        self.agent_name = my_name;
        self
    }

    pub fn openai_api_key(&mut self, openai_api_key: String) -> &mut Self {
        self.openai_api_key = openai_api_key;
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

    pub async fn build(&mut self) -> Agent {
        let llm_engine = LLMEngine::new(self.openai_api_key.to_owned());

        let memory_engine = MemoryEngine::new(self.database_url.to_owned()).await;

        Agent::new(
            self.agent_name.to_owned(),
            DefaultInteraction {
                user_name: self.default_user_name.to_owned(),
                constitution: self.default_constitution.to_owned(),
                memory_size: self.default_memory_size,
            },
            llm_engine,
            memory_engine,
        )
    }
}
