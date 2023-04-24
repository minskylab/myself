use std::{marker::PhantomData, str::FromStr};

use chrono::Utc;
use sqlx::{postgres::PgPool, query};
use uuid::Uuid;

use crate::{
    agent::{Agent, DefaultInteraction},
    llm::LLMEngine,
    sdk::interactions::{
        Interaction, InteractionBlock, InteractionBlockRole, Meta, WithAgent, WithoutAgent,
    },
};

use super::{engine::new_postgres_pool, models::migrate_database_with_pg_pool};

#[derive(Debug, Clone)]
pub struct MemoryEngine {
    pool: PgPool,
}

impl MemoryEngine {
    pub async fn new(database_url: String) -> Self {
        let pool = new_postgres_pool(database_url).await;

        migrate_database_with_pg_pool(&pool).await;

        Self { pool }
    }

    pub async fn new_defaults() -> Self {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or("sqlite://sqlite.db".to_string());

        Self::new(database_url).await
    }

    pub async fn new_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        let interaction = Interaction::new(user_name, constitution, memory_size);

        let res = query!(
            r#"
            INSERT INTO interactions (id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory
            "#,
            interaction.id,
            interaction.created_at.naive_utc(),
            interaction.updated_at.naive_utc(),
            interaction.user_name,
            interaction.default_long_term_memory_size as i32,
            interaction.constitution,
            interaction.short_term_memory,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Interaction::<WithoutAgent> {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            user_name: res.user_name,
            default_long_term_memory_size: res.default_long_term_memory_size as usize,
            short_term_memory: res.short_term_memory,
            constitution: res.constitution,
            state: PhantomData,
            agent: None,
        }
    }

    pub async fn new_interaction_with_agent(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
        agent: &Agent,
    ) -> Interaction<WithAgent> {
        let mut interaction = self
            .new_interaction(user_name, constitution, memory_size)
            .await;

        interaction.with_agent(agent.to_owned()) // TODO: Check if it can be optimized
    }

    pub async fn update_constitution(&mut self, id: Uuid, constitution: String) -> Interaction {
        todo!()
    }

    pub async fn append_to_long_term_memory(
        &mut self,
        id: Uuid,
        interaction_user_name: String,
        interaction_user_message: String,
        agent_name: String,
        agent_response: String,
    ) -> (InteractionBlock, InteractionBlock) {
        let user_interaction_block = query!(
            r#"
            INSERT INTO interaction_blocks (id, created_at, updated_at, interaction_id, role, content, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at, updated_at, interaction_id, role, content, name
            "#,
            Uuid::new_v4(),
            Utc::now().naive_utc(),
            Utc::now().naive_utc(),
            id,
            InteractionBlockRole::User.as_str(),
            interaction_user_message,
            interaction_user_name,
        )
        .fetch_one(&self.pool)
        .await
        .map(|res| InteractionBlock {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            interaction_id: res.interaction_id,
            role: InteractionBlockRole::from_str(&res.role).unwrap(),
            content: res.content,
            name: res.name,
        })
        .unwrap();

        let agent_interaction_block = query!(
            r#"
            INSERT INTO interaction_blocks (id, created_at, updated_at, interaction_id, role, content, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at, updated_at, interaction_id, role, content, name
            "#,
            Uuid::new_v4(),
            Utc::now().naive_utc(),
            Utc::now().naive_utc(),
            id,
            InteractionBlockRole::Agent.as_str(),
            agent_response,
            agent_name,
        )
        .fetch_one(&self.pool)
        .await
        .map(|res| InteractionBlock {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            interaction_id: res.interaction_id,
            role: InteractionBlockRole::from_str(&res.role).unwrap(),
            content: res.content,
            name: res.name,
        })
        .unwrap();

        (user_interaction_block, agent_interaction_block)
    }

    pub async fn set_short_term_memory(
        &mut self,
        interaction_id: Uuid,
        memory: String,
    ) -> Interaction {
        todo!()
    }

    pub async fn get_meta(&mut self) -> Option<Meta> {
        todo!()
    }

    pub async fn get_interaction(&mut self, id: Uuid) -> Option<Interaction> {
        let res = query!(
            r#"
            SELECT id, created_at, updated_at, user_name, default_long_term_memory_size, constitution, short_term_memory
            FROM interactions
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();

        res.map(|res| Interaction::<WithoutAgent> {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            user_name: res.user_name,
            short_term_memory: res.short_term_memory,
            default_long_term_memory_size: res.default_long_term_memory_size as usize,
            constitution: res.constitution,
            state: PhantomData,
            agent: None,
        })
    }

    pub async fn set_default_interaction(&mut self, id: Uuid) -> Meta {
        todo!()
    }

    pub async fn get_default_interaction(&mut self) -> Option<Interaction> {
        todo!()
    }

    pub async fn get_or_create_default_interaction(
        &mut self,
        user_name: String,
        constitution: String,
        memory_size: usize,
    ) -> Interaction {
        todo!()
    }

    pub async fn get_all_interactions(&mut self) -> Vec<Interaction> {
        todo!()
    }

    pub async fn new_agent(
        &mut self,
        name: String,
        default_user_name: String,
        default_constitution: String,
        default_memory_size: usize,
        llm_engine: LLMEngine,
        memory_engine: MemoryEngine,
    ) -> Agent {
        let new_id = Uuid::new_v4();
        let res = query!(
            r#"
            INSERT INTO agents (id, name, default_interaction_user_name, default_interaction_constitution, default_interaction_memory_size)
            VALUES ($1, $2, $3, $4, $5) RETURNING id, name, default_interaction_user_name, default_interaction_constitution, default_interaction_memory_size
            "#,
            new_id,
            name,
            default_user_name,
            default_constitution,
            default_memory_size as i32,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Agent::new(
            res.id,
            res.name,
            DefaultInteraction {
                user_name: res.default_interaction_user_name,
                constitution: res.default_interaction_constitution,
                memory_size: res.default_interaction_memory_size as usize,
            },
            llm_engine,
            memory_engine,
        )
    }

    pub async fn get_interaction_long_term_memory(
        &self,
        interaction_id: Uuid,
        limit: usize,
    ) -> Vec<InteractionBlock> {
        query!(
            r#"
            SELECT id, created_at, updated_at, name, interaction_id, role, content
            FROM interaction_blocks
            WHERE interaction_id = $1
            ORDER BY created_at ASC
            LIMIT $2
            "#,
            interaction_id,
            limit as i64,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
        .iter()
        .map(|res| InteractionBlock {
            id: res.id,
            created_at: res.created_at.and_local_timezone(Utc).unwrap(),
            updated_at: res.updated_at.and_local_timezone(Utc).unwrap(),
            name: res.name.clone(),
            interaction_id: res.interaction_id,
            role: InteractionBlockRole::from_str(res.role.as_str()).unwrap(),
            content: res.content.clone(),
        })
        .collect()
    }
}

impl Interaction<WithAgent> {
    pub async fn long_term_memory(&self, memory_size: usize) -> Vec<InteractionBlock> {
        self.agent
            .clone()
            .unwrap()
            .memory_engine()
            .get_interaction_long_term_memory(self.id, memory_size)
            .await
    }
}

impl Interaction<WithoutAgent> {
    pub async fn long_term_memory(
        &self,
        agent: &mut Agent,
        memory_size: usize,
    ) -> Vec<InteractionBlock> {
        agent
            .memory_engine()
            .get_interaction_long_term_memory(self.id, memory_size)
            .await
    }
}

impl Interaction<WithAgent> {
    pub async fn interact(&mut self, message: &String) -> Option<String> {
        self.agent.clone().unwrap().interact(self.id, message).await
    }
}
