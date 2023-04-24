use std::marker::PhantomData;

// use chrono::{ Utc};
use crate::{agent::Agent, database::memory::MemoryEngine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub trait InteractionState {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithAgent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithoutAgent;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct WithMemorySupport;

impl InteractionState for WithAgent {}
impl InteractionState for WithoutAgent {}
// impl InteractionState for WithMemorySupport {}

#[derive(Clone, Debug)]
pub struct Interaction<State = WithoutAgent>
where
    State: InteractionState,
{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub user_name: String,

    pub short_term_memory: Option<String>,
    pub short_term_memory_size: usize,

    pub state: PhantomData<State>,

    pub agent: Option<Box<Agent>>,
}

#[derive(Clone, Debug)]
pub struct Meta {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub default_interaction: Option<Uuid>,
}

pub enum InteractionBlockRole {
    System,
    User,
    Agent,
}

pub struct InteractionBlock {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub role: InteractionBlockRole,
    pub content: String,

    pub name: Option<String>,

    interaction_id: Uuid,
}

impl Interaction<WithAgent> {
    fn long_term_memory(&self) -> Vec<InteractionBlock> {
        // match &self.short_term_memory {
        //     Some(short_term_memory) => short_term_memory.to_string(),
        //     None => "".to_string(),
        // }

        // self.memory.unwrap().get_long_term_memory(self.id.clone())
        todo!()
    }
}

impl Interaction<WithAgent> {
    pub async fn interact(&mut self, message: &String) -> Option<String> {
        self.agent
            .clone()
            .unwrap()
            .interact(self.id.clone(), message)
            .await
    }
}

impl Interaction {
    pub fn new(
        user_name: String,
        long_term_memory_init: String,
        short_term_memory_size: usize,
    ) -> Self {
        Self {
            user_name,
            // long_term_memory: long_term_memory_init,
            short_term_memory_size,
            ..Default::default()
        }
    }

    pub fn new_with_agent(
        user_name: String,
        long_term_memory_init: String,
        short_term_memory_size: usize,
        agent: Agent,
    ) -> Interaction<WithAgent> {
        Interaction::<WithAgent> {
            user_name,
            // long_term_memory: long_term_memory_init,
            short_term_memory_size,
            agent: Some(Box::new(agent)),
            state: PhantomData,
            ..Default::default()
        }
    }

    pub fn set_agent(&mut self, agent: Agent) -> Interaction<WithAgent> {
        self.agent = Some(Box::new(agent));
        Interaction::<WithAgent> {
            id: self.id.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
            user_name: self.user_name.clone(),
            // long_term_memory: self.long_term_memory.clone(),
            short_term_memory: self.short_term_memory.clone(),
            short_term_memory_size: self.short_term_memory_size,
            agent: self.agent.clone(),
            state: PhantomData,
        }
    }
}

impl<State> Default for Interaction<State>
where
    State: InteractionState,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),

            user_name: "".to_string(),

            // long_term_memory: "".to_string(),
            short_term_memory: None,
            short_term_memory_size: 0,
            agent: None,
            state: PhantomData,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),

            default_interaction: None,
        }
    }
}
