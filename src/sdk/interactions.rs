use std::{marker::PhantomData, str::FromStr};

use crate::{
    agent::Agent,
    backend::{AgentBackend, OpenAIBackend},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

pub trait InteractionState {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithAgent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WithoutAgent;

impl InteractionState for WithAgent {}
impl InteractionState for WithoutAgent {}

#[derive(Clone, Debug)]
pub struct Interaction<Backend = OpenAIBackend, State = WithoutAgent>
where
    State: InteractionState,
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub user_name: String,

    pub constitution: String,

    pub short_term_memory: String,

    pub default_long_term_memory_size: usize,

    pub state: PhantomData<State>,

    pub agent: Option<Box<Agent<Backend>>>,
}

#[derive(Clone, Debug)]
pub struct Meta {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub default_interaction_id: Uuid,
}

pub enum InteractionBlockRole {
    System,
    User,
    Agent,
}

impl InteractionBlockRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            InteractionBlockRole::System => "system",
            InteractionBlockRole::User => "user",
            InteractionBlockRole::Agent => "agent",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl FromStr for InteractionBlockRole {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(InteractionBlockRole::System),
            "user" => Ok(InteractionBlockRole::User),
            "agent" => Ok(InteractionBlockRole::Agent),
            _ => Err("Invalid interaction block role"),
        }
    }
}

pub struct InteractionBlock {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub role: InteractionBlockRole,
    pub content: String,

    pub name: Option<String>,

    pub interaction_id: Uuid,
}

impl InteractionBlock {
    pub fn new(
        role: InteractionBlockRole,
        content: String,
        name: Option<String>,
        interaction_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            role,
            content,
            name,
            interaction_id,
        }
    }
}

impl<Backend, State> Interaction<Backend, State>
where
    State: InteractionState,
    Backend: AgentBackend + Sized + Default + Clone,
{
    pub fn new(
        user_name: String,
        constitution: String,
        default_long_term_memory_size: usize,
    ) -> Self {
        Self {
            user_name,
            constitution,
            default_long_term_memory_size,
            ..Default::default()
        }
    }

    pub fn new_with_agent(
        user_name: String,
        _long_term_memory_init: String,
        default_long_term_memory_size: usize,
        agent: Agent<Backend>,
    ) -> Interaction<Backend, WithAgent>
    where
        Backend: AgentBackend + Sized + Default + Clone,
    {
        Interaction::<Backend, WithAgent> {
            user_name,
            default_long_term_memory_size,
            agent: Some(Box::new(agent)),
            state: PhantomData,
            ..Default::default()
        }
    }

    pub fn with_agent(&mut self, agent: Agent<Backend>) -> Interaction<Backend, WithAgent>
    where
        Backend: AgentBackend + Sized + Default + Clone,
    {
        Interaction::<Backend, WithAgent> {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user_name: self.user_name.clone(),
            constitution: self.constitution.clone(),
            short_term_memory: self.short_term_memory.clone(),
            default_long_term_memory_size: self.default_long_term_memory_size,
            agent: Some(Box::new(agent)),
            state: PhantomData,
        }
    }
}

impl<Backend, State> Default for Interaction<Backend, State>
where
    Backend: AgentBackend + Sized + Default + Clone,
    State: InteractionState,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),

            user_name: "".to_string(),

            constitution: "".to_string(),
            short_term_memory: "".to_string(),
            default_long_term_memory_size: 0,
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

            default_interaction_id: Uuid::new_v4(),
        }
    }
}
