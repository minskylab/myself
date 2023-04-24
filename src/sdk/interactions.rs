use std::{marker::PhantomData, str::FromStr};

use crate::agent::Agent;
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
pub struct Interaction<State = WithoutAgent>
where
    State: InteractionState,
{
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub user_name: String,

    pub constitution: String,

    pub short_term_memory: String,

    pub default_long_term_memory_size: usize,

    pub state: PhantomData<State>,

    pub agent: Option<Box<Agent>>,
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

impl Interaction {
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
        long_term_memory_init: String,
        default_long_term_memory_size: usize,
        agent: Agent,
    ) -> Interaction<WithAgent> {
        Interaction::<WithAgent> {
            user_name,
            default_long_term_memory_size,
            agent: Some(Box::new(agent)),
            state: PhantomData,
            ..Default::default()
        }
    }

    pub fn with_agent(&mut self, agent: Agent) -> Interaction<WithAgent> {
        self.agent = Some(Box::new(agent));
        Interaction::<WithAgent> {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            user_name: self.user_name.clone(),
            constitution: self.constitution.clone(),
            short_term_memory: self.short_term_memory.clone(),
            default_long_term_memory_size: self.default_long_term_memory_size,
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
